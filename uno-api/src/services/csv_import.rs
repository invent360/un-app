//! CSV import parsing and validation.

use chrono::{DateTime, NaiveDate, Utc};

use crate::error::ApiError;
use crate::models::{ImportError, LicenseInput, SplitType};

/// Parsed CSV result.
#[derive(Debug, Clone)]
pub struct ParsedCsv {
    /// Parsed license inputs.
    pub licenses: Vec<LicenseInput>,
    /// Parse errors (non-fatal).
    pub errors: Vec<ImportError>,
    /// Total rows processed.
    pub total_rows: usize,
}

/// Parse CSV data into license inputs.
///
/// Expected columns (with header):
/// - lease_code (required)
/// - valid_from (required, format: YYYY-MM-DD)
/// - valid_to (required, format: YYYY-MM-DD)
/// - split_type (required, format: "50:50", "55:45", or "60:40")
///
/// Or without header: lease_code, valid_from, valid_to, split_type.
/// Date-only values use ISO YYYY-MM-DD. RFC3339 timestamps preserve their offset.
/// This is the legacy split import contract; it does not create v2 agreements.
pub fn parse_csv(csv_data: &str, has_header: bool) -> Result<ParsedCsv, ApiError> {
    let mut reader = csv::ReaderBuilder::new()
        .has_headers(has_header)
        .flexible(false)
        .trim(csv::Trim::All)
        .from_reader(csv_data.as_bytes());

    let mut licenses = Vec::new();
    let mut errors = Vec::new();
    let mut total_rows = 0;

    // Get column indices from header if present
    let col_indices = if has_header {
        let headers = reader
            .headers()
            .map_err(|e| ApiError::CsvParse(format!("Failed to read headers: {}", e)))?
            .clone();
        Some(get_column_indices(&headers))
    } else {
        None
    };

    for (idx, result) in reader.records().enumerate() {
        total_rows += 1;
        let row_num = idx + if has_header { 2 } else { 1 }; // 1-based, accounting for header

        match result {
            Ok(record) => match parse_record(&record, row_num, &col_indices) {
                Ok(input) => licenses.push(input),
                Err(err) => errors.push(err),
            },
            Err(e) => {
                errors.push(ImportError::new(
                    row_num,
                    format!("Failed to parse row: {}", e),
                ));
            }
        }
    }

    Ok(ParsedCsv {
        licenses,
        errors,
        total_rows,
    })
}

/// Column indices for parsing.
#[derive(Debug, Clone, Default)]
struct ColumnIndices {
    lease_code: Option<usize>,
    valid_from: Option<usize>,
    valid_to: Option<usize>,
    split_type: Option<usize>,
}

fn get_column_indices(headers: &csv::StringRecord) -> ColumnIndices {
    let mut indices = ColumnIndices::default();

    for (idx, header) in headers.iter().enumerate() {
        let header_lower = header.to_lowercase();
        let header_normalized = header_lower.replace(['-', ' '], "_");

        match header_normalized.as_str() {
            "lease_code" | "leasecode" | "code" => indices.lease_code = Some(idx),
            "valid_from" | "validfrom" | "from" | "start" | "start_date" => {
                indices.valid_from = Some(idx)
            }
            "valid_to" | "validto" | "to" | "end" | "end_date" | "expiry" => {
                indices.valid_to = Some(idx)
            }
            "split_type" | "splittype" | "split" | "type" => indices.split_type = Some(idx),
            _ => {}
        }
    }

    indices
}

fn parse_record(
    record: &csv::StringRecord,
    row_num: usize,
    col_indices: &Option<ColumnIndices>,
) -> Result<LicenseInput, ImportError> {
    let (lease_code_idx, valid_from_idx, valid_to_idx, split_type_idx) =
        if let Some(indices) = col_indices {
            (
                indices.lease_code,
                indices
                    .valid_from
                    .ok_or_else(|| ImportError::new(row_num, "Missing valid_from header"))?,
                indices
                    .valid_to
                    .ok_or_else(|| ImportError::new(row_num, "Missing valid_to header"))?,
                indices
                    .split_type
                    .ok_or_else(|| ImportError::new(row_num, "Missing split_type header"))?,
            )
        } else {
            // Headerless records require a real credential in the first column.
            (Some(0), 1, 2, 3)
        };

    // Parse valid_from
    let valid_from_str = record
        .get(valid_from_idx)
        .ok_or_else(|| ImportError::new(row_num, "Missing valid_from column"))?
        .trim();

    let valid_from = parse_date(valid_from_str).map_err(|_| {
        ImportError::new(
            row_num,
            format!("Invalid valid_from date: {}", valid_from_str),
        )
        .with_column("valid_from")
    })?;

    // Parse valid_to
    let valid_to_str = record
        .get(valid_to_idx)
        .ok_or_else(|| ImportError::new(row_num, "Missing valid_to column"))?
        .trim();

    let valid_to = parse_date(valid_to_str).map_err(|_| {
        ImportError::new(row_num, format!("Invalid valid_to date: {}", valid_to_str))
            .with_column("valid_to")
    })?;

    // Validate date range
    if valid_to <= valid_from {
        return Err(ImportError::new(
            row_num,
            "valid_to must be after valid_from",
        ));
    }

    // Parse split_type
    let split_type_str = record
        .get(split_type_idx)
        .ok_or_else(|| ImportError::new(row_num, "Missing split_type column"))?
        .trim();

    let split_type = SplitType::from_str(split_type_str).ok_or_else(|| {
        ImportError::new(
            row_num,
            format!(
                "Invalid split_type '{}'. Must be 50:50, 55:45, or 60:40",
                split_type_str
            ),
        )
        .with_column("split_type")
    })?;

    // Required lease_code
    let lease_code = lease_code_idx
        .and_then(|idx| record.get(idx))
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .ok_or_else(|| ImportError::new(row_num, "Missing or empty lease_code"))?;

    Ok(LicenseInput {
        id: None, // CSV import doesn't support custom IDs
        lease_code,
        valid_from,
        valid_to,
        split_type,
        // R5-06: Optional fields not provided in CSV
        uno_share_pct: None,
        ulo_share_pct: None,
        agent_share_pct: None,
        source_system: Some("csv_import".to_string()),
        source_version: None,
        source_record_id: None,
    })
}

/// Parse a date string in various formats.
fn parse_date(s: &str) -> Result<DateTime<Utc>, ()> {
    // Try ISO format first (YYYY-MM-DD)
    if let Ok(date) = NaiveDate::parse_from_str(s, "%Y-%m-%d") {
        return Ok(date.and_hms_opt(0, 0, 0).unwrap().and_utc());
    }

    if let Ok(timestamp) = DateTime::parse_from_rfc3339(s) {
        return Ok(timestamp.with_timezone(&Utc));
    }

    Err(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_csv_with_header() {
        let csv = "lease_code,valid_from,valid_to,split_type\nreal-code-a,2024-01-01,2024-12-31,50:50\nreal-code-b,2024-06-01,2025-06-01,60:40";
        let result = parse_csv(csv, true).unwrap();

        assert_eq!(result.licenses.len(), 2);
        assert_eq!(result.errors.len(), 0);
        assert_eq!(result.licenses[0].split_type, SplitType::Split5050);
        assert_eq!(result.licenses[1].split_type, SplitType::Split6040);
    }

    #[test]
    fn test_parse_csv_without_header() {
        let csv =
            "real-code-a,2024-01-01,2024-12-31,50:50\nreal-code-b,2024-06-01,2025-06-01,55:45";
        let result = parse_csv(csv, false).unwrap();

        assert_eq!(result.licenses.len(), 2);
    }

    #[test]
    fn test_parse_csv_invalid_split_type() {
        let csv = "valid_from,valid_to,split_type\n2024-01-01,2024-12-31,70:30";
        let result = parse_csv(csv, true).unwrap();

        assert_eq!(result.licenses.len(), 0);
        assert_eq!(result.errors.len(), 1);
        assert!(result.errors[0].message.contains("Invalid split_type"));
    }

    #[test]
    fn test_parse_csv_invalid_date_range() {
        let csv = "valid_from,valid_to,split_type\n2024-12-31,2024-01-01,50:50";
        let result = parse_csv(csv, true).unwrap();

        assert_eq!(result.licenses.len(), 0);
        assert_eq!(result.errors.len(), 1);
        assert!(result.errors[0].message.contains("valid_to must be after"));
    }
}
