//! Host-local media storage. All operations are relative to retained directory
//! descriptors, refuse symlinks, and never accept caller filesystem URLs.
use crate::{
    config::LocalConfig,
    error::{Result, StorageError},
    traits::FileStorageClient,
    types::{FileRetrievalResult, FileUploadResult, UploadError, UploadOptions},
};
use async_trait::async_trait;
use image::{ImageFormat, ImageReader, Limits};
use rustix::fs::{self, AtFlags, Mode, OFlags};
use std::{
    fs::File,
    io::{Cursor, Read, Write},
    path::Path,
};

pub struct LocalStorageClient {
    config: LocalConfig,
    root: File,
}

fn io_error(error: rustix::io::Errno) -> StorageError {
    if error == rustix::io::Errno::NOENT {
        return StorageError::NotFound("Object not found".into());
    }
    if error == rustix::io::Errno::LOOP {
        return StorageError::SymlinkRejected("Symlinks are not permitted".into());
    }
    std::io::Error::from(error).into()
}

impl LocalStorageClient {
    pub fn new(config: LocalConfig) -> Result<Self> {
        // Provisioning owns directory creation. A missing mount must never be replaced
        // by an empty container directory, including during development.
        let root = File::from(
            fs::open(
                &config.base_path,
                OFlags::RDONLY | OFlags::DIRECTORY | OFlags::NOFOLLOW | OFlags::CLOEXEC,
                Mode::empty(),
            )
            .map_err(io_error)?,
        );
        let production = std::env::var("RUST_ENV")
            .is_ok_and(|v| v.eq_ignore_ascii_case("production"))
            || std::env::var("LEPTOS_ENV").is_ok_and(|v| v.eq_ignore_ascii_case("PROD"));
        if production {
            let expected = std::env::var("FILE_STORAGE_VOLUME_ID")
                .ok()
                .filter(|id| !id.trim().is_empty())
                .ok_or_else(|| {
                    StorageError::ConfigError("FILE_STORAGE_VOLUME_ID is required".into())
                })?;
            let mut marker = File::from(
                fs::openat(
                    &root,
                    ".uno-volume",
                    OFlags::RDONLY | OFlags::NOFOLLOW | OFlags::CLOEXEC,
                    Mode::empty(),
                )
                .map_err(io_error)?,
            );
            let mut identity = String::new();
            Read::by_ref(&mut marker)
                .take(256)
                .read_to_string(&mut identity)?;
            if identity.trim() != expected {
                return Err(StorageError::ConfigError(
                    "Media volume identity does not match".into(),
                ));
            }
        }
        Ok(Self { config, root })
    }

    fn validate_resource_id(&self, id: &str) -> Result<()> {
        if id.is_empty()
            || id.len() > 128
            || !id
                .bytes()
                .all(|c| c.is_ascii_alphanumeric() || c == b'-' || c == b'_')
        {
            return Err(StorageError::InvalidResourceId(
                "Invalid opaque resource identifier".into(),
            ));
        }
        Ok(())
    }

    fn directory(&self, id: &str, create: bool) -> Result<File> {
        self.validate_resource_id(id)?;
        if create {
            match fs::mkdirat(&self.root, id, Mode::RUSR | Mode::WUSR | Mode::XUSR) {
                Ok(()) | Err(rustix::io::Errno::EXIST) => {}
                Err(error) => return Err(io_error(error)),
            }
        }
        fs::openat(
            &self.root,
            id,
            OFlags::RDONLY | OFlags::DIRECTORY | OFlags::NOFOLLOW | OFlags::CLOEXEC,
            Mode::empty(),
        )
        .map(File::from)
        .map_err(io_error)
    }

    fn validate_filename(&self, name: &str) -> Result<&'static str> {
        if name.is_empty() || name.len() > 255 || name.contains(['/', '\\']) || name.contains("..")
        {
            return Err(StorageError::InvalidUrl("Invalid filename".into()));
        }
        match Path::new(name)
            .extension()
            .and_then(|ext| ext.to_str())
            .map(str::to_ascii_lowercase)
            .as_deref()
        {
            Some("jpg" | "jpeg") => Ok("jpg"),
            Some("png") => Ok("png"),
            Some("webp") => Ok("webp"),
            _ => Err(StorageError::InvalidFileType(
                "Only decoded JPEG, PNG and WebP images are supported".into(),
            )),
        }
    }

    fn approved_image(
        &self,
        bytes: &[u8],
        filename: &str,
        mime: &str,
        options: &Option<UploadOptions>,
    ) -> Result<(Vec<u8>, &'static str)> {
        let limit = options
            .as_ref()
            .and_then(|o| o.max_size)
            .unwrap_or(self.config.max_file_size)
            .min(self.config.max_file_size);
        if bytes.len() as u64 > limit {
            return Err(StorageError::FileTooLarge {
                size: bytes.len() as u64,
                limit,
            });
        }
        let extension = self.validate_filename(filename)?;
        let mut reader = ImageReader::new(Cursor::new(bytes)).with_guessed_format()?;
        let format = reader
            .format()
            .ok_or_else(|| StorageError::InvalidFileType("Unrecognized image content".into()))?;
        let (expected_mime, expected_extension) = match format {
            ImageFormat::Jpeg => ("image/jpeg", "jpg"),
            ImageFormat::Png => ("image/png", "png"),
            ImageFormat::WebP => ("image/webp", "webp"),
            _ => {
                return Err(StorageError::InvalidFileType(
                    "Unsupported image format".into(),
                ))
            }
        };
        let permitted = |types: &[String]| {
            types.is_empty() || types.iter().any(|t| t == expected_mime || t == "image/*")
        };
        if mime != expected_mime
            || extension != expected_extension
            || !permitted(&self.config.allowed_mime_types)
            || options
                .as_ref()
                .is_some_and(|o| !permitted(&o.allowed_types))
        {
            return Err(StorageError::InvalidFileType(
                "Filename, declared type and decoded content must agree".into(),
            ));
        }
        let mut limits = Limits::default();
        limits.max_image_width = Some(4096);
        limits.max_image_height = Some(4096);
        limits.max_alloc = Some(64 * 1024 * 1024);
        reader.limits(limits);
        let decoded = reader
            .decode()
            .map_err(|error| StorageError::InvalidFileType(error.to_string()))?;
        let mut output = Cursor::new(Vec::new());
        decoded
            .write_to(&mut output, format)
            .map_err(|error| StorageError::ProcessingError(error.to_string()))?;
        let output = output.into_inner();
        if output.len() as u64 > limit {
            return Err(StorageError::FileTooLarge {
                size: output.len() as u64,
                limit,
            });
        }
        Ok((output, extension))
    }

    fn write_object(&self, directory: &File, name: &str, data: &[u8]) -> Result<()> {
        let staging = format!(".upload-{}", uuid::Uuid::new_v4());
        let mut file = File::from(
            fs::openat(
                directory,
                &staging,
                OFlags::WRONLY | OFlags::CREATE | OFlags::EXCL | OFlags::NOFOLLOW | OFlags::CLOEXEC,
                Mode::RUSR | Mode::WUSR,
            )
            .map_err(io_error)?,
        );
        let result = (|| {
            file.write_all(data)?;
            file.sync_all()?;
            // Names are generated UUIDs; a caller can never select an overwrite target.
            fs::renameat(directory, &staging, directory, name).map_err(io_error)?;
            directory.sync_all()?;
            Ok(())
        })();
        if result.is_err() {
            let _ = fs::unlinkat(directory, &staging, AtFlags::empty());
        }
        result
    }

    fn names(&self, directory: &File) -> Result<Vec<String>> {
        let entries = fs::Dir::read_from(directory).map_err(io_error)?;
        let mut names = Vec::new();
        for entry in entries {
            let entry = entry.map_err(io_error)?;
            let Some(name) = entry.file_name().to_str().ok() else {
                continue;
            };
            if self.validate_filename(name).is_ok() {
                names.push(name.to_string());
            }
        }
        names.sort();
        Ok(names)
    }

    fn url(&self, id: &str, name: &str) -> String {
        format!("local://{id}/{name}")
    }
    fn display_url(&self, id: &str, name: &str) -> String {
        format!("{}/{id}/{name}", self.config.base_url.trim_end_matches('/'))
    }
}

#[async_trait]
impl FileStorageClient for LocalStorageClient {
    async fn upload_files(
        &self,
        resource_id: &str,
        files: Vec<(Vec<u8>, String, String)>,
        options: Option<UploadOptions>,
    ) -> Result<FileUploadResult> {
        let directory = self.directory(resource_id, true)?;
        let mut result = FileUploadResult::empty();
        for (bytes, original, mime) in files {
            let approved = self.approved_image(&bytes, &original, &mime, &options);
            match approved {
                Ok((bytes, extension)) => {
                    let name = format!("{}.{}", uuid::Uuid::new_v4(), extension);
                    match self.write_object(&directory, &name, &bytes) {
                        Ok(()) => {
                            result.total_bytes += bytes.len() as u64;
                            result.storage_urls.push(self.url(resource_id, &name));
                            result
                                .display_urls
                                .push(self.display_url(resource_id, &name));
                        }
                        Err(error) => result.errors.push(UploadError {
                            filename: original,
                            error: error.to_string(),
                        }),
                    }
                }
                Err(error) => result.errors.push(UploadError {
                    filename: original,
                    error: error.to_string(),
                }),
            }
        }
        result.uploaded_count = result.storage_urls.len();
        Ok(result)
    }

    async fn upload_file(
        &self,
        id: &str,
        bytes: Vec<u8>,
        filename: &str,
        mime: &str,
    ) -> Result<String> {
        let result = self
            .upload_files(
                id,
                vec![(bytes, filename.to_string(), mime.to_string())],
                None,
            )
            .await?;
        result.storage_urls.into_iter().next().ok_or_else(|| {
            StorageError::UploadError(
                result
                    .errors
                    .first()
                    .map(|error| error.error.clone())
                    .unwrap_or_else(|| "Upload failed".into()),
            )
        })
    }

    async fn get_files(&self, id: &str) -> Result<FileRetrievalResult> {
        let directory = match self.directory(id, false) {
            Ok(directory) => directory,
            Err(StorageError::NotFound(_)) => return Ok(FileRetrievalResult::empty()),
            Err(error) => return Err(error),
        };
        let mut result = FileRetrievalResult::empty();
        for name in self.names(&directory)? {
            let fd = fs::openat(
                &directory,
                &name,
                OFlags::RDONLY | OFlags::NOFOLLOW | OFlags::CLOEXEC | OFlags::NONBLOCK,
                Mode::empty(),
            )
            .map_err(io_error)?;
            if !File::from(fd).metadata()?.is_file() {
                return Err(StorageError::InvalidUrl(
                    "Object is not a regular file".into(),
                ));
            }
            result.storage_urls.push(self.url(id, &name));
            result.display_urls.push(self.display_url(id, &name));
        }
        result.count = result.storage_urls.len();
        Ok(result)
    }

    async fn get_file(&self, url: &str) -> Result<Vec<u8>> {
        let (id, name) = self.parse_storage_url(url).ok_or_else(|| {
            StorageError::InvalidUrl("Expected an opaque local object URL".into())
        })?;
        let directory = self.directory(&id, false)?;
        let fd = fs::openat(
            &directory,
            &name,
            OFlags::RDONLY | OFlags::NOFOLLOW | OFlags::CLOEXEC | OFlags::NONBLOCK,
            Mode::empty(),
        )
        .map_err(io_error)?;
        let mut file = File::from(fd);
        let metadata = file.metadata()?;
        if !metadata.is_file() {
            return Err(StorageError::InvalidUrl(
                "Object is not a regular file".into(),
            ));
        }
        let limit = self.config.max_file_size;
        if metadata.len() > limit {
            return Err(StorageError::FileTooLarge {
                size: metadata.len(),
                limit,
            });
        }
        let mut bytes = Vec::new();
        Read::by_ref(&mut file)
            .take(limit + 1)
            .read_to_end(&mut bytes)?;
        if bytes.len() as u64 > limit {
            return Err(StorageError::FileTooLarge {
                size: bytes.len() as u64,
                limit,
            });
        }
        Ok(bytes)
    }

    async fn delete_files(&self, id: &str) -> Result<()> {
        let directory = self.directory(id, false)?;
        // Never recursively delete caller-selected directories. Only validated regular
        // objects directly in an already-open resource directory may be unlinked.
        for name in self.names(&directory)? {
            let fd = fs::openat(
                &directory,
                &name,
                OFlags::RDONLY | OFlags::NOFOLLOW | OFlags::CLOEXEC | OFlags::NONBLOCK,
                Mode::empty(),
            )
            .map_err(io_error)?;
            if !File::from(fd).metadata()?.is_file() {
                return Err(StorageError::InvalidUrl(
                    "Object is not a regular file".into(),
                ));
            }
            fs::unlinkat(&directory, &name, AtFlags::empty()).map_err(io_error)?;
        }
        directory.sync_all()?;
        // Leave the directory in place to avoid races with new uploads.
        Ok(())
    }

    async fn delete_file(&self, url: &str) -> Result<()> {
        let (id, name) = self.parse_storage_url(url).ok_or_else(|| {
            StorageError::InvalidUrl("Expected an opaque local object URL".into())
        })?;
        let directory = self.directory(&id, false)?;
        let fd = fs::openat(
            &directory,
            &name,
            OFlags::RDONLY | OFlags::NOFOLLOW | OFlags::CLOEXEC | OFlags::NONBLOCK,
            Mode::empty(),
        )
        .map_err(io_error)?;
        if !File::from(fd).metadata()?.is_file() {
            return Err(StorageError::InvalidUrl(
                "Object is not a regular file".into(),
            ));
        }
        fs::unlinkat(&directory, &name, AtFlags::empty()).map_err(io_error)?;
        directory.sync_all()?;
        Ok(())
    }

    fn storage_to_display_urls(&self, urls: &[String]) -> Vec<String> {
        urls.iter()
            .map(|url| self.storage_to_display_url(url))
            .collect()
    }
    fn storage_to_display_url(&self, url: &str) -> String {
        self.parse_storage_url(url)
            .map(|(id, name)| self.display_url(&id, &name))
            .unwrap_or_default()
    }
    fn parse_storage_url(&self, url: &str) -> Option<(String, String)> {
        let (id, name) = url.strip_prefix("local://")?.split_once('/')?;
        self.validate_resource_id(id).ok()?;
        self.validate_filename(name).ok()?;
        Some((id.to_string(), name.to_string()))
    }
    async fn is_configured(&self) -> bool {
        self.root.metadata().is_ok_and(|metadata| metadata.is_dir())
    }
    fn backend_name(&self) -> &'static str {
        "local_filesystem"
    }
}
