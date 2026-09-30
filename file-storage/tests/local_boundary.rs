#![cfg(feature = "local")]

use file_storage::{FileStorageClient, LocalConfig, LocalStorageClient};
use image::{DynamicImage, ImageFormat};
use std::io::Cursor;
use tempfile::TempDir;

fn client(root: &TempDir) -> LocalStorageClient {
    LocalStorageClient::new(LocalConfig {
        base_path: root.path().to_string_lossy().into_owned(),
        base_url: "/files".into(),
        max_file_size: 10 * 1024 * 1024,
        allowed_mime_types: vec!["image/png".into()],
    })
    .unwrap()
}

fn png(width: u32, height: u32) -> Vec<u8> {
    let mut bytes = Cursor::new(Vec::new());
    DynamicImage::new_rgb8(width, height)
        .write_to(&mut bytes, ImageFormat::Png)
        .unwrap();
    bytes.into_inner()
}

#[tokio::test]
async fn decoded_images_roundtrip_with_immutable_opaque_names() {
    let root = TempDir::new().unwrap();
    let client = client(&root);
    let mut bytes = png(2, 2);
    bytes.extend_from_slice(b"<script>appended payload</script>");
    let first = client
        .upload_file("resource", bytes.clone(), "image.png", "image/png")
        .await
        .unwrap();
    let second = client
        .upload_file("resource", bytes, "image.png", "image/png")
        .await
        .unwrap();
    assert_ne!(first, second);
    assert!(first.starts_with("local://resource/"));
    let stored = client.get_file(&first).await.unwrap();
    assert!(!stored.windows(8).any(|bytes| bytes == b"<script>"));
    assert!(image::load_from_memory(&stored).is_ok());
    assert_eq!(client.get_files("resource").await.unwrap().count, 2);
    client.delete_file(&first).await.unwrap();
    assert!(client.get_file(&first).await.is_err());
    assert!(client.get_file(&second).await.is_ok());
}

#[tokio::test]
async fn rejects_fake_images_mismatched_types_and_excessive_dimensions() {
    let root = TempDir::new().unwrap();
    let client = client(&root);
    for (bytes, filename, mime) in [
        (b"<html>script</html>".to_vec(), "image.png", "image/png"),
        (b"<svg></svg>".to_vec(), "image.svg", "image/svg+xml"),
        (png(2, 2), "image.jpg", "image/jpeg"),
        (png(2, 2), "image.png", "text/html"),
        (png(4097, 1), "image.png", "image/png"),
    ] {
        assert!(client
            .upload_file("resource", bytes, filename, mime)
            .await
            .is_err());
    }
    assert_eq!(client.get_files("resource").await.unwrap().count, 0);
}

#[tokio::test]
async fn rejects_paths_and_filesystem_urls() {
    let root = TempDir::new().unwrap();
    let client = client(&root);
    for resource in ["../escape", "resource/child", "", "a\\b"] {
        assert!(client
            .upload_file(resource, png(1, 1), "image.png", "image/png")
            .await
            .is_err());
    }
    for url in [
        "file:///etc/passwd",
        "local://resource/../image.png",
        "local://../image.png",
        "local://resource/child/image.png",
    ] {
        assert!(client.get_file(url).await.is_err());
        assert!(client.delete_file(url).await.is_err());
        assert!(client.parse_storage_url(url).is_none());
    }
}

#[cfg(unix)]
#[tokio::test]
async fn refuses_symlink_resources_and_objects_without_touching_targets() {
    use std::os::unix::fs::symlink;
    let root = TempDir::new().unwrap();
    let outside = TempDir::new().unwrap();
    let sentinel = outside.path().join("sentinel.png");
    std::fs::write(&sentinel, b"outside sentinel").unwrap();
    symlink(outside.path(), root.path().join("escape")).unwrap();
    let client = client(&root);
    assert!(client
        .upload_file("escape", png(1, 1), "image.png", "image/png")
        .await
        .is_err());
    assert!(client
        .get_file("local://escape/sentinel.png")
        .await
        .is_err());
    assert!(client.delete_files("escape").await.is_err());
    std::fs::create_dir(root.path().join("resource")).unwrap();
    symlink(&sentinel, root.path().join("resource/sentinel.png")).unwrap();
    assert!(client
        .get_file("local://resource/sentinel.png")
        .await
        .is_err());
    assert!(client
        .delete_file("local://resource/sentinel.png")
        .await
        .is_err());
    assert_eq!(std::fs::read(sentinel).unwrap(), b"outside sentinel");
    assert_eq!(std::fs::read_dir(outside.path()).unwrap().count(), 1);
}

#[cfg(unix)]
#[tokio::test]
async fn retained_root_descriptor_prevents_path_replacement_escape() {
    use std::os::unix::fs::symlink;
    let parent = TempDir::new().unwrap();
    let outside = TempDir::new().unwrap();
    let original = parent.path().join("media");
    std::fs::create_dir(&original).unwrap();
    let client = LocalStorageClient::new(LocalConfig {
        base_path: original.to_string_lossy().into_owned(),
        base_url: "/files".into(),
        max_file_size: 1024 * 1024,
        allowed_mime_types: vec!["image/png".into()],
    })
    .unwrap();
    std::fs::rename(&original, parent.path().join("retained")).unwrap();
    symlink(outside.path(), &original).unwrap();
    let url = client
        .upload_file("resource", png(1, 1), "image.png", "image/png")
        .await
        .unwrap();
    assert!(client.get_file(&url).await.is_ok());
    assert_eq!(std::fs::read_dir(outside.path()).unwrap().count(), 0);
    assert!(parent.path().join("retained/resource").is_dir());
}

#[test]
fn missing_root_is_an_error_and_is_never_created() {
    let parent = TempDir::new().unwrap();
    let missing = parent.path().join("missing");
    assert!(LocalStorageClient::new(LocalConfig {
        base_path: missing.to_string_lossy().into_owned(),
        base_url: "/files".into(),
        max_file_size: 1024,
        allowed_mime_types: vec![],
    })
    .is_err());
    assert!(!missing.exists());
}
