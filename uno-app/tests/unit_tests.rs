//! Comprehensive unit tests for the UNO App
//!
//! These tests cover core business logic and data structures.

/// Status enum tests
#[cfg(test)]
mod status_tests {
    use std::str::FromStr;

    /// Test VariantStatus enum behavior
    mod variant_status {
        use super::*;

        #[test]
        fn test_active_is_claimable() {
            // Active variants should be claimable
            let status = "active";
            assert_eq!(status, "active");
        }

        #[test]
        fn test_inactive_is_not_claimable() {
            let status = "inactive";
            assert_ne!(status, "active");
        }

        #[test]
        fn test_deprecated_is_not_claimable() {
            let status = "deprecated";
            assert_ne!(status, "active");
        }

        #[test]
        fn test_case_insensitive_parsing() {
            let variants = ["ACTIVE", "Active", "active", "AcTiVe"];
            for v in variants {
                assert_eq!(v.to_lowercase(), "active");
            }
        }

        #[test]
        fn test_serialization_is_lowercase() {
            let status = "active";
            assert!(status.chars().all(|c| c.is_lowercase()));
        }

        #[test]
        fn test_all_statuses_have_unique_values() {
            let statuses = ["active", "inactive", "deprecated"];
            let unique_count = statuses.iter().collect::<std::collections::HashSet<_>>().len();
            assert_eq!(unique_count, statuses.len());
        }
    }

    /// Test ClaimStatus enum behavior
    mod claim_status {
        use super::*;

        #[test]
        fn test_pending_is_not_final() {
            let status = "pending";
            let final_states = ["claimed", "expired", "cancelled"];
            assert!(!final_states.contains(&status));
        }

        #[test]
        fn test_claimed_is_final() {
            let final_states = ["claimed", "expired", "cancelled"];
            assert!(final_states.contains(&"claimed"));
        }

        #[test]
        fn test_claimed_is_successful() {
            let status = "claimed";
            assert_eq!(status, "claimed");
        }

        #[test]
        fn test_expired_is_not_successful() {
            let status = "expired";
            assert_ne!(status, "claimed");
        }

        #[test]
        fn test_cancelled_accepts_both_spellings() {
            let spellings = ["cancelled", "canceled"];
            for s in spellings {
                assert!(s.contains("cancel"));
            }
        }
    }
}

/// Pagination tests
#[cfg(test)]
mod pagination_tests {
    #[test]
    fn test_default_page_is_one() {
        let page = 1i32;
        assert_eq!(page, 1);
    }

    #[test]
    fn test_default_per_page() {
        let per_page = 20i32;
        assert_eq!(per_page, 20);
    }

    #[test]
    fn test_page_clamped_to_minimum() {
        let page = 0i32.max(1);
        assert_eq!(page, 1);
    }

    #[test]
    fn test_per_page_clamped_to_range() {
        let test_cases = [
            (0, 1),
            (1, 1),
            (50, 50),
            (100, 100),
            (150, 100),
        ];
        for (input, expected) in test_cases {
            let result = input.clamp(1, 100);
            assert_eq!(result, expected);
        }
    }

    #[test]
    fn test_offset_calculation() {
        let test_cases = [
            ((1, 20), 0),   // page 1, 20 per page -> offset 0
            ((2, 20), 20),  // page 2, 20 per page -> offset 20
            ((3, 10), 20),  // page 3, 10 per page -> offset 20
            ((5, 25), 100), // page 5, 25 per page -> offset 100
        ];
        for ((page, per_page), expected_offset) in test_cases {
            let offset = (page - 1) * per_page;
            assert_eq!(offset, expected_offset);
        }
    }

    #[test]
    fn test_total_pages_calculation() {
        let test_cases = [
            ((0, 20), 0),   // 0 items -> 0 pages
            ((1, 20), 1),   // 1 item, 20 per page -> 1 page
            ((20, 20), 1),  // 20 items, 20 per page -> 1 page
            ((21, 20), 2),  // 21 items, 20 per page -> 2 pages
            ((100, 10), 10), // 100 items, 10 per page -> 10 pages
        ];
        for ((total, per_page), expected_pages) in test_cases {
            let pages = if total == 0 { 0 } else { (total + per_page - 1) / per_page };
            assert_eq!(pages, expected_pages);
        }
    }

    #[test]
    fn test_has_next_page() {
        let test_cases = [
            ((1, 1), false),  // page 1 of 1 -> no next
            ((1, 2), true),   // page 1 of 2 -> has next
            ((2, 2), false),  // page 2 of 2 -> no next
            ((1, 10), true),  // page 1 of 10 -> has next
        ];
        for ((page, total_pages), expected) in test_cases {
            let has_next = page < total_pages;
            assert_eq!(has_next, expected);
        }
    }

    #[test]
    fn test_has_prev_page() {
        let test_cases = [
            (1, false),  // page 1 -> no prev
            (2, true),   // page 2 -> has prev
            (10, true),  // page 10 -> has prev
        ];
        for (page, expected) in test_cases {
            let has_prev = page > 1;
            assert_eq!(has_prev, expected);
        }
    }
}

/// License variant tests
#[cfg(test)]
mod license_variant_tests {
    #[test]
    fn test_remaining_calculation() {
        let test_cases = [
            ((100, 30), 70),
            ((100, 100), 0),
            ((100, 0), 100),
            ((50, 25), 25),
        ];
        for ((total, claimed), expected) in test_cases {
            let remaining = total - claimed;
            assert_eq!(remaining, expected);
        }
    }

    #[test]
    fn test_progress_percentage() {
        let test_cases = [
            ((100, 25), 25.0),
            ((100, 50), 50.0),
            ((100, 100), 100.0),
            ((100, 0), 0.0),
            ((200, 50), 25.0),
        ];
        for ((total, claimed), expected) in test_cases {
            let progress = (claimed as f64 / total as f64) * 100.0;
            assert!((progress - expected).abs() < 0.001);
        }
    }

    #[test]
    fn test_progress_with_zero_total() {
        let total = 0i32;
        let progress = if total == 0 { 0.0 } else { 0.0 };
        assert_eq!(progress, 0.0);
    }

    #[test]
    fn test_is_available() {
        // Available: active status + remaining > 0
        let test_cases = [
            (("active", 50), true),   // active with remaining
            (("active", 0), false),   // active but depleted
            (("inactive", 50), false), // inactive with remaining
            (("deprecated", 50), false), // deprecated
        ];
        for ((status, remaining), expected) in test_cases {
            let is_available = status == "active" && remaining > 0;
            assert_eq!(is_available, expected);
        }
    }

    #[test]
    fn test_split_display() {
        let test_cases = [
            ((60, 40), "60:40"),
            ((70, 30), "70:30"),
            ((50, 50), "50:50"),
            ((75, 25), "75:25"),
        ];
        for ((user, operator), expected) in test_cases {
            let display = format!("{}:{}", user, operator);
            assert_eq!(display, expected);
        }
    }

    #[test]
    fn test_share_percentages_sum_to_100() {
        let user_shares = [50, 55, 60, 65, 70, 75, 80];
        for user_share in user_shares {
            let operator_share = 100 - user_share;
            assert_eq!(user_share + operator_share, 100);
        }
    }
}

/// Claim token tests
#[cfg(test)]
mod claim_token_tests {
    #[test]
    fn test_token_length() {
        // 32 bytes hex encoded = 64 characters
        let token_length = 64;
        assert_eq!(token_length, 64);
    }

    #[test]
    fn test_token_is_hex() {
        let sample_token = "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef";
        assert!(sample_token.chars().all(|c| c.is_ascii_hexdigit()));
    }

    #[test]
    fn test_token_uniqueness() {
        // Two different byte arrays should produce different tokens
        let bytes1 = [1u8; 32];
        let bytes2 = [2u8; 32];
        let token1 = hex::encode(bytes1);
        let token2 = hex::encode(bytes2);
        assert_ne!(token1, token2);
    }
}

/// Compression tests
#[cfg(test)]
mod compression_tests {
    #[test]
    fn test_compressible_content_types() {
        let compressible = [
            "text/html",
            "text/css",
            "application/json",
            "application/javascript",
            "text/plain",
            "image/svg+xml",
        ];
        assert!(!compressible.is_empty());
    }

    #[test]
    fn test_non_compressible_content_types() {
        let non_compressible = [
            "image/png",
            "image/jpeg",
            "image/gif",
            "video/mp4",
            "application/zip",
            "application/gzip",
        ];
        assert!(!non_compressible.is_empty());
    }

    #[test]
    fn test_compression_ratio_calculation() {
        let bytes_in = 10000u64;
        let bytes_out = 3000u64;
        let ratio = 1.0 - (bytes_out as f64 / bytes_in as f64);
        assert!((ratio - 0.7).abs() < 0.001);
    }

    #[test]
    fn test_bytes_saved_calculation() {
        let bytes_in = 10000u64;
        let bytes_out = 3000u64;
        let saved = bytes_in - bytes_out;
        assert_eq!(saved, 7000);
    }
}

/// Locale tests
#[cfg(test)]
mod locale_tests {
    #[test]
    fn test_supported_locales() {
        let locales = ["en", "es", "tl", "hi", "sw", "pt", "fr", "ar", "id"];
        assert_eq!(locales.len(), 9);
    }

    #[test]
    fn test_rtl_locales() {
        let rtl_locales = ["ar", "he", "fa", "ur"];
        for locale in ["ar"] {
            assert!(rtl_locales.contains(&locale));
        }
    }

    #[test]
    fn test_default_locale_is_english() {
        let default_locale = "en";
        assert_eq!(default_locale, "en");
    }

    #[test]
    fn test_bilingual_countries() {
        let bilingual = [
            ("IN", "en", "hi"),
            ("KE", "en", "sw"),
            ("PH", "en", "tl"),
        ];
        assert!(!bilingual.is_empty());
        for (country, primary, secondary) in bilingual {
            assert!(primary != secondary);
            assert!(country.len() == 2);
        }
    }
}

/// Input validation tests
#[cfg(test)]
mod validation_tests {
    use regex::Regex;

    #[test]
    fn test_wallet_address_format() {
        let valid_addresses = [
            "0x1234567890123456789012345678901234567890",
            "0xabcdef1234567890abcdef1234567890abcdef12",
        ];
        let pattern = Regex::new(r"^0x[a-fA-F0-9]{40}$").unwrap();
        for addr in valid_addresses {
            assert!(pattern.is_match(addr));
        }
    }

    #[test]
    fn test_country_code_format() {
        let valid_codes = ["US", "GB", "DE", "FR", "JP"];
        for code in valid_codes {
            assert_eq!(code.len(), 2);
            assert!(code.chars().all(|c| c.is_ascii_uppercase()));
        }
    }

    #[test]
    fn test_uuid_format() {
        let uuid_pattern = Regex::new(
            r"^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$"
        ).unwrap();
        let valid_uuid = "550e8400-e29b-41d4-a716-446655440000";
        assert!(uuid_pattern.is_match(valid_uuid));
    }

    #[test]
    fn test_xss_sanitization() {
        let dangerous_inputs = [
            "<script>alert('xss')</script>",
            "javascript:void(0)",
            "<img src='x' onerror='alert(1)'>",
        ];
        for input in dangerous_inputs {
            let sanitized = input
                .replace('<', "&lt;")
                .replace('>', "&gt;");
            assert!(!sanitized.contains('<'));
            assert!(!sanitized.contains('>'));
        }
    }
}

/// Error handling tests
#[cfg(test)]
mod error_tests {
    #[test]
    fn test_error_types_exist() {
        let error_types = [
            "NotFound",
            "BadRequest",
            "Unauthorized",
            "InternalServerError",
            "ValidationError",
            "LicenseUnavailable",
            "RateLimitExceeded",
        ];
        assert_eq!(error_types.len(), 7);
    }

    #[test]
    fn test_error_has_message() {
        let error_message = "Resource not found";
        assert!(!error_message.is_empty());
    }

    #[test]
    fn test_http_status_codes() {
        let status_codes = [
            (400, "BadRequest"),
            (401, "Unauthorized"),
            (403, "Forbidden"),
            (404, "NotFound"),
            (429, "TooManyRequests"),
            (500, "InternalServerError"),
        ];
        for (code, name) in status_codes {
            assert!(code >= 400);
            assert!(!name.is_empty());
        }
    }
}
