//! Migration type definitions

/// Represents a single migration
#[derive(Debug, Clone)]
pub struct Migration {
    pub version: u32,
    pub name: String,
    pub up_sql: String,
    pub down_sql: Option<String>,
}

/// Migration direction
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MigrationDirection {
    Up,
    Down,
}

/// Result of migration execution
#[derive(Debug)]
pub struct MigrationResult {
    pub version: u32,
    pub name: String,
    pub direction: MigrationDirection,
    pub success: bool,
    pub error: Option<String>,
}
