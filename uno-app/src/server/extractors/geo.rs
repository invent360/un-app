//! IP Geolocation extractor

use actix_web::{FromRequest, HttpRequest, dev::Payload};
use std::future::{Ready, ready};
use crate::locales;

/// Geolocation data extracted from IP address
#[derive(Debug, Clone, Default)]
pub struct GeoLocation {
    pub ip: String,
    pub country_code: Option<String>,
    pub country_name: Option<String>,
    pub city: Option<String>,
    pub region: Option<String>,
    pub is_bilingual_country: bool,
    pub primary_locale: Option<String>,
    pub secondary_locale: Option<String>,
}

impl GeoLocation {
    /// Create from IP address with basic country detection
    pub fn from_ip(ip: &str) -> Self {
        let country = detect_country_from_ip(ip);
        let country_code = country.map(|(code, _)| code.to_string());

        // Check if this is a bilingual country
        let (is_bilingual, primary, secondary) = if let Some(ref code) = country_code {
            if let Some((p, s)) = locales::get_bilingual_options(code) {
                (true, Some(p.to_string()), Some(s.to_string()))
            } else {
                (false, None, None)
            }
        } else {
            (false, None, None)
        };

        Self {
            ip: ip.to_string(),
            country_code,
            country_name: country.map(|(_, name)| name.to_string()),
            city: None,
            region: None,
            is_bilingual_country: is_bilingual,
            primary_locale: primary,
            secondary_locale: secondary,
        }
    }

    /// Get the recommended default locale for this location
    pub fn default_locale(&self) -> &str {
        if let Some(ref code) = self.country_code {
            locales::get_country_locale(code)
        } else {
            "en"
        }
    }
}

/// Extract client IP from request
fn extract_client_ip(req: &HttpRequest) -> String {
    // Check X-Forwarded-For header first (for proxies)
    if let Some(forwarded) = req.headers().get("X-Forwarded-For") {
        if let Ok(forwarded_str) = forwarded.to_str() {
            // Take the first IP in the chain (original client)
            if let Some(ip) = forwarded_str.split(',').next() {
                return ip.trim().to_string();
            }
        }
    }

    // Check X-Real-IP header
    if let Some(real_ip) = req.headers().get("X-Real-IP") {
        if let Ok(ip) = real_ip.to_str() {
            return ip.trim().to_string();
        }
    }

    // Fall back to connection info
    req.connection_info()
        .realip_remote_addr()
        .unwrap_or("unknown")
        .to_string()
}

/// Basic country detection from IP ranges
/// In production, use a proper GeoIP database or API
fn detect_country_from_ip(ip: &str) -> Option<(&'static str, &'static str)> {
    // Parse IP to check ranges
    let parts: Vec<u8> = ip
        .split('.')
        .filter_map(|p| p.parse().ok())
        .collect();

    if parts.len() != 4 {
        return None;
    }

    let first_octet = parts[0];

    // Very basic geographic hints based on IP allocation
    // This is NOT accurate - use MaxMind GeoIP or similar in production
    match first_octet {
        // Private ranges - localhost
        10 | 127 => Some(("XX", "Local/Private")),
        172 if parts[1] >= 16 && parts[1] <= 31 => Some(("XX", "Local/Private")),
        192 if parts[1] == 168 => Some(("XX", "Local/Private")),
        
        // Some rough regional allocations (very approximate)
        1..=9 => Some(("US", "United States")),
        11..=30 => Some(("US", "United States")),
        41..=42 => Some(("ZA", "South Africa")),
        58..=61 => Some(("CN", "China")),
        62..=90 => Some(("EU", "Europe")),
        101..=126 => Some(("JP", "Japan")),
        128..=140 => Some(("US", "United States")),
        141..=150 => Some(("EU", "Europe")),
        175..=180 => Some(("AU", "Australia")),
        190..=191 => Some(("BR", "Brazil")),
        200..=201 => Some(("BR", "Brazil")),
        202..=210 => Some(("IN", "India")),
        
        // Default unknown
        _ => None,
    }
}

impl FromRequest for GeoLocation {
    type Error = actix_web::Error;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _payload: &mut Payload) -> Self::Future {
        let ip = extract_client_ip(req);
        ready(Ok(GeoLocation::from_ip(&ip)))
    }
}

/// Get country name from code
pub fn country_name_from_code(code: &str) -> &'static str {
    match code.to_uppercase().as_str() {
        "US" => "United States",
        "GB" => "United Kingdom",
        "CA" => "Canada",
        "AU" => "Australia",
        "DE" => "Germany",
        "FR" => "France",
        "JP" => "Japan",
        "CN" => "China",
        "IN" => "India",
        "BR" => "Brazil",
        "MX" => "Mexico",
        "ES" => "Spain",
        "IT" => "Italy",
        "NL" => "Netherlands",
        "PH" => "Philippines",
        "ID" => "Indonesia",
        "TH" => "Thailand",
        "VN" => "Vietnam",
        "MY" => "Malaysia",
        "SG" => "Singapore",
        "KR" => "South Korea",
        "TW" => "Taiwan",
        "HK" => "Hong Kong",
        "ZA" => "South Africa",
        "NG" => "Nigeria",
        "KE" => "Kenya",
        "EG" => "Egypt",
        "AE" => "United Arab Emirates",
        "SA" => "Saudi Arabia",
        "PK" => "Pakistan",
        "BD" => "Bangladesh",
        "RU" => "Russia",
        "UA" => "Ukraine",
        "PL" => "Poland",
        "TR" => "Turkey",
        "AR" => "Argentina",
        "CL" => "Chile",
        "CO" => "Colombia",
        "PE" => "Peru",
        "XX" => "Local/Private",
        _ => "Unknown",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_localhost_detection() {
        let geo = GeoLocation::from_ip("127.0.0.1");
        assert_eq!(geo.country_code, Some("XX".to_string()));
    }

    #[test]
    fn test_private_ip_detection() {
        let geo = GeoLocation::from_ip("192.168.1.1");
        assert_eq!(geo.country_code, Some("XX".to_string()));
    }

    #[test]
    fn test_country_name_lookup() {
        assert_eq!(country_name_from_code("US"), "United States");
        assert_eq!(country_name_from_code("PH"), "Philippines");
    }
}
