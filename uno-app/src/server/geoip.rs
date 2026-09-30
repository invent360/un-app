//! GeoIP lookup service using MaxMind GeoLite2 database

use maxminddb::{PathElement, Reader};
use std::net::IpAddr;
use std::path::Path;
use std::sync::Arc;
use tracing::{info, warn};

/// GeoIP lookup service
#[derive(Clone)]
pub struct GeoIpService {
    reader: Arc<Reader<Vec<u8>>>,
}

impl GeoIpService {
    /// Create a new GeoIP service from a database file path
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self, maxminddb::MaxMindDbError> {
        let reader = Reader::open_readfile(path)?;
        info!("GeoIP database loaded successfully");
        Ok(Self {
            reader: Arc::new(reader),
        })
    }

    /// Look up country code for an IP address
    /// Returns ISO 3166-1 alpha-2 country code (e.g., "US", "DE", "GB")
    pub fn lookup_country(&self, ip: &str) -> Option<String> {
        // Parse the IP address
        let ip_addr: IpAddr = match ip.parse() {
            Ok(addr) => addr,
            Err(_) => {
                // Try to extract IP from socket address (e.g., "192.168.1.1:12345")
                ip.split(':').next().and_then(|s| s.parse().ok())?
            }
        };

        // Look up the country
        match self.reader.lookup(ip_addr) {
            Ok(result) => result
                .decode_path(&[PathElement::Key("country"), PathElement::Key("iso_code")])
                .ok()
                .flatten(),
            Err(_) => None,
        }
    }
}

/// Optional GeoIP service wrapper for when database is not available
#[derive(Clone)]
pub struct OptionalGeoIp(Option<GeoIpService>);

impl OptionalGeoIp {
    /// Create from environment variable GEOIP_DATABASE_PATH
    pub fn from_env() -> Self {
        match std::env::var("GEOIP_DATABASE_PATH") {
            Ok(path) => match GeoIpService::new(&path) {
                Ok(service) => {
                    info!(path = %path, "GeoIP service initialized");
                    Self(Some(service))
                }
                Err(e) => {
                    warn!(error = %e, path = %path, "Failed to load GeoIP database");
                    Self(None)
                }
            },
            Err(_) => {
                info!("GEOIP_DATABASE_PATH not set - country detection disabled");
                Self(None)
            }
        }
    }

    /// Look up country code, returning None if service not available
    pub fn lookup_country(&self, ip: &str) -> Option<String> {
        self.0.as_ref().and_then(|s| s.lookup_country(ip))
    }

    /// Check if GeoIP service is available
    pub fn is_available(&self) -> bool {
        self.0.is_some()
    }
}
