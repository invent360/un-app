//! Loads migration files from disk

use super::types::Migration;
use crate::db::error::{DbError, DbResult};
use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

pub struct MigrationLoader {
    migrations_dir: String,
}

impl MigrationLoader {
    pub fn new(migrations_dir: &str) -> Self {
        Self {
            migrations_dir: migrations_dir.to_string(),
        }
    }

    /// Load all migrations from the migrations directory
    pub fn load_all(&self) -> DbResult<Vec<Migration>> {
        let path = Path::new(&self.migrations_dir);
        if !path.exists() {
            return Err(DbError::Migration(format!(
                "Migrations directory not found: {}",
                self.migrations_dir
            )));
        }

        let mut migrations_map: BTreeMap<u32, (String, Option<String>, Option<String>)> =
            BTreeMap::new();

        // Read all .cql files
        for entry in fs::read_dir(path).map_err(|e| DbError::Migration(e.to_string()))? {
            let entry = entry.map_err(|e| DbError::Migration(e.to_string()))?;
            let file_name = entry.file_name().to_string_lossy().to_string();

            if !file_name.ends_with(".cql") {
                continue;
            }

            // Parse filename: 0001_name.up.cql or 0001_name.down.cql
            if let Some((version, name, direction)) = self.parse_filename(&file_name) {
                let content =
                    fs::read_to_string(entry.path()).map_err(|e| DbError::Migration(e.to_string()))?;

                let entry = migrations_map
                    .entry(version)
                    .or_insert_with(|| (name, None, None));

                match direction.as_str() {
                    "up" => entry.1 = Some(content),
                    "down" => entry.2 = Some(content),
                    _ => {}
                }
            }
        }

        // Convert to Migration structs
        let migrations: Vec<Migration> = migrations_map
            .into_iter()
            .filter_map(|(version, (name, up_sql, down_sql))| {
                up_sql.map(|up| Migration {
                    version,
                    name,
                    up_sql: up,
                    down_sql,
                })
            })
            .collect();

        Ok(migrations)
    }

    fn parse_filename(&self, filename: &str) -> Option<(u32, String, String)> {
        // Expected format: 0001_agents.up.cql
        let without_ext = filename.strip_suffix(".cql")?;
        let (base, direction) = if without_ext.ends_with(".up") {
            (without_ext.strip_suffix(".up")?, "up")
        } else if without_ext.ends_with(".down") {
            (without_ext.strip_suffix(".down")?, "down")
        } else {
            return None;
        };

        let underscore_idx = base.find('_')?;
        let version_str = &base[..underscore_idx];
        let name = &base[underscore_idx + 1..];

        let version = version_str.parse::<u32>().ok()?;
        Some((version, name.to_string(), direction.to_string()))
    }
}
