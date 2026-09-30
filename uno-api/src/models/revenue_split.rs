//! Revenue split model using basis points for precise allocation.
//!
//! This module implements the 50/40/10 revenue split model with:
//! - ULO (User License Owner): License holder's share
//! - UNO (U Network Operator): Platform/operator share
//! - Referral: Referrer's commission share
//!
//! All values are stored in basis points (1 bp = 0.01%, 10000 bp = 100%)
//! to avoid floating-point precision issues.

use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Total basis points representing 100%.
pub const TOTAL_BASIS_POINTS: u32 = 10_000;

/// Default ULO (user) share: 50% = 5000 bp
pub const DEFAULT_ULO_BPS: u32 = 5_000;

/// Default UNO (operator) share: 40% = 4000 bp
pub const DEFAULT_UNO_BPS: u32 = 4_000;

/// Default referral share: 10% = 1000 bp
pub const DEFAULT_REFERRAL_BPS: u32 = 1_000;

/// Errors that can occur during revenue split operations.
#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub enum RevenueSplitError {
    /// Basis points don't sum to 10000.
    #[error("Basis points must sum to 10000, got {0}")]
    InvalidSum(u32),

    /// Individual share exceeds maximum.
    #[error("Share {name} ({value} bps) exceeds maximum ({max} bps)")]
    ShareExceedsMax { name: &'static str, value: u32, max: u32 },

    /// Arithmetic overflow during allocation.
    #[error("Arithmetic overflow during allocation")]
    ArithmeticOverflow,

    /// Pool amount is negative or invalid.
    #[error("Invalid pool amount: {0}")]
    InvalidPoolAmount(i64),
}

/// Revenue split configuration in basis points.
///
/// Represents the revenue distribution between:
/// - ULO (User License Owner): The person who claimed/owns the license
/// - UNO (U Network Operator): The platform operator
/// - Referral: The referrer who brought the user (if any)
///
/// All values are in basis points where 10000 bp = 100%.
///
/// # Example
///
/// ```
/// use uno_api::models::RevenueSplit;
///
/// // Create the default 50/40/10 split
/// let split = RevenueSplit::default();
/// assert_eq!(split.ulo_bps, 5000);  // 50%
/// assert_eq!(split.uno_bps, 4000);  // 40%
/// assert_eq!(split.referral_bps, 1000);  // 10%
///
/// // Allocate 1000000 micros (1 USD)
/// let allocation = split.allocate(1_000_000).unwrap();
/// assert_eq!(allocation.ulo_micros, 500_000);  // $0.50
/// assert_eq!(allocation.uno_micros, 400_000);  // $0.40
/// assert_eq!(allocation.referral_micros, 100_000);  // $0.10
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct RevenueSplit {
    /// ULO (User License Owner) share in basis points.
    pub ulo_bps: u32,
    /// UNO (U Network Operator) share in basis points.
    pub uno_bps: u32,
    /// Referral share in basis points.
    pub referral_bps: u32,
    /// Agreement version this split belongs to.
    #[serde(default)]
    pub agreement_version: Option<i32>,
}

impl Default for RevenueSplit {
    /// Creates the canonical 50/40/10 split.
    fn default() -> Self {
        Self {
            ulo_bps: DEFAULT_ULO_BPS,
            uno_bps: DEFAULT_UNO_BPS,
            referral_bps: DEFAULT_REFERRAL_BPS,
            agreement_version: Some(1),
        }
    }
}

impl RevenueSplit {
    /// Create a new revenue split with validation.
    ///
    /// # Errors
    ///
    /// Returns an error if the basis points don't sum to 10000.
    pub fn new(ulo_bps: u32, uno_bps: u32, referral_bps: u32) -> Result<Self, RevenueSplitError> {
        let split = Self {
            ulo_bps,
            uno_bps,
            referral_bps,
            agreement_version: None,
        };
        split.validate()?;
        Ok(split)
    }

    /// Create a new revenue split with agreement version.
    pub fn with_version(
        ulo_bps: u32,
        uno_bps: u32,
        referral_bps: u32,
        version: i32,
    ) -> Result<Self, RevenueSplitError> {
        let mut split = Self::new(ulo_bps, uno_bps, referral_bps)?;
        split.agreement_version = Some(version);
        Ok(split)
    }

    /// Create a split without referral (redistributes to UNO).
    ///
    /// When there's no referrer, the referral portion goes to UNO.
    pub fn without_referral(ulo_bps: u32) -> Result<Self, RevenueSplitError> {
        let uno_bps = TOTAL_BASIS_POINTS.saturating_sub(ulo_bps);
        Self::new(ulo_bps, uno_bps, 0)
    }

    /// Validate that basis points sum to 10000.
    pub fn validate(&self) -> Result<(), RevenueSplitError> {
        let sum = self.ulo_bps + self.uno_bps + self.referral_bps;
        if sum != TOTAL_BASIS_POINTS {
            return Err(RevenueSplitError::InvalidSum(sum));
        }

        // Optional: validate individual shares don't exceed reasonable limits
        if self.ulo_bps > TOTAL_BASIS_POINTS {
            return Err(RevenueSplitError::ShareExceedsMax {
                name: "ulo_bps",
                value: self.ulo_bps,
                max: TOTAL_BASIS_POINTS,
            });
        }

        Ok(())
    }

    /// Allocate a pool amount in micros using checked integer arithmetic.
    ///
    /// Uses the following rounding policy:
    /// 1. Calculate each share with integer division
    /// 2. Calculate the remainder
    /// 3. Assign the remainder to UNO (operator) to ensure exact reconciliation
    ///
    /// # Arguments
    ///
    /// * `pool_micros` - The total pool amount in micros (1 USD = 1,000,000 micros)
    ///
    /// # Returns
    ///
    /// An `Allocation` struct with the exact amounts for each party.
    ///
    /// # Errors
    ///
    /// Returns an error if arithmetic overflow occurs or pool is negative.
    pub fn allocate(&self, pool_micros: i64) -> Result<Allocation, RevenueSplitError> {
        if pool_micros < 0 {
            return Err(RevenueSplitError::InvalidPoolAmount(pool_micros));
        }

        let pool = pool_micros as u64;

        // Calculate shares using integer arithmetic
        // Formula: share = (pool * bps) / 10000
        let ulo_micros = pool
            .checked_mul(self.ulo_bps as u64)
            .ok_or(RevenueSplitError::ArithmeticOverflow)?
            / TOTAL_BASIS_POINTS as u64;

        let referral_micros = pool
            .checked_mul(self.referral_bps as u64)
            .ok_or(RevenueSplitError::ArithmeticOverflow)?
            / TOTAL_BASIS_POINTS as u64;

        // UNO gets calculated share plus any remainder (rounding policy)
        let uno_calculated = pool
            .checked_mul(self.uno_bps as u64)
            .ok_or(RevenueSplitError::ArithmeticOverflow)?
            / TOTAL_BASIS_POINTS as u64;

        // Calculate remainder and assign to UNO
        let subtotal = ulo_micros + referral_micros + uno_calculated;
        let remainder = pool.saturating_sub(subtotal);
        let uno_micros = uno_calculated + remainder;

        // Verify reconciliation
        debug_assert_eq!(
            ulo_micros + uno_micros + referral_micros,
            pool,
            "Revenue allocation must reconcile exactly"
        );

        Ok(Allocation {
            pool_micros: pool as i64,
            ulo_micros: ulo_micros as i64,
            uno_micros: uno_micros as i64,
            referral_micros: referral_micros as i64,
            reserve_micros: 0, // R3-07: Reserve is 0 when referral is present
            ulo_bps: self.ulo_bps,
            uno_bps: self.uno_bps,
            referral_bps: self.referral_bps,
            remainder_micros: remainder as i64,
        })
    }

    /// R3-07: Allocate for a license without a referral.
    ///
    /// When there's no referrer, the referral portion goes to reserve,
    /// NOT to UNO. This ensures the 50/40/10 split is preserved as terms,
    /// with the unclaimed 10% held separately.
    ///
    /// # Arguments
    ///
    /// * `pool_micros` - The total pool amount in micros
    ///
    /// # Returns
    ///
    /// An `Allocation` with referral_micros=0 and reserve_micros=(10% share)
    pub fn allocate_without_referral(&self, pool_micros: i64) -> Result<Allocation, RevenueSplitError> {
        if pool_micros < 0 {
            return Err(RevenueSplitError::InvalidPoolAmount(pool_micros));
        }

        let pool = pool_micros as u64;

        // Calculate shares using integer arithmetic
        let ulo_micros = pool
            .checked_mul(self.ulo_bps as u64)
            .ok_or(RevenueSplitError::ArithmeticOverflow)?
            / TOTAL_BASIS_POINTS as u64;

        let uno_calculated = pool
            .checked_mul(self.uno_bps as u64)
            .ok_or(RevenueSplitError::ArithmeticOverflow)?
            / TOTAL_BASIS_POINTS as u64;

        // R3-07: Referral share goes to reserve, not UNO
        let reserve_micros = pool
            .checked_mul(self.referral_bps as u64)
            .ok_or(RevenueSplitError::ArithmeticOverflow)?
            / TOTAL_BASIS_POINTS as u64;

        // Calculate remainder and assign to UNO
        let subtotal = ulo_micros + uno_calculated + reserve_micros;
        let remainder = pool.saturating_sub(subtotal);
        let uno_micros = uno_calculated + remainder;

        // Verify reconciliation
        debug_assert_eq!(
            ulo_micros + uno_micros + reserve_micros,
            pool,
            "Revenue allocation must reconcile exactly"
        );

        Ok(Allocation {
            pool_micros: pool as i64,
            ulo_micros: ulo_micros as i64,
            uno_micros: uno_micros as i64,
            referral_micros: 0, // No referral
            reserve_micros: reserve_micros as i64, // R3-07: Goes to reserve
            ulo_bps: self.ulo_bps,
            uno_bps: self.uno_bps,
            referral_bps: self.referral_bps,
            remainder_micros: remainder as i64,
        })
    }

    /// Convert ULO basis points to percentage.
    pub fn ulo_percentage(&self) -> f64 {
        self.ulo_bps as f64 / 100.0
    }

    /// Convert UNO basis points to percentage.
    pub fn uno_percentage(&self) -> f64 {
        self.uno_bps as f64 / 100.0
    }

    /// Convert referral basis points to percentage.
    pub fn referral_percentage(&self) -> f64 {
        self.referral_bps as f64 / 100.0
    }

    /// Display as ratio string (e.g., "50/40/10").
    pub fn display(&self) -> String {
        format!(
            "{}/{}/{}",
            self.ulo_bps / 100,
            self.uno_bps / 100,
            self.referral_bps / 100
        )
    }
}

/// Result of allocating a pool using a revenue split.
///
/// All amounts are in micros (1 USD = 1,000,000 micros).
/// The allocation is guaranteed to reconcile exactly:
/// `ulo_micros + uno_micros + referral_micros + reserve_micros == pool_micros`
///
/// R3-07: When no referral is attributed, the referral share goes to reserve,
/// not to UNO. Reserve funds are held separately.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Allocation {
    /// Original pool amount in micros.
    pub pool_micros: i64,
    /// ULO (User License Owner) allocation in micros.
    pub ulo_micros: i64,
    /// UNO (U Network Operator) allocation in micros.
    pub uno_micros: i64,
    /// Referral allocation in micros.
    pub referral_micros: i64,
    /// R3-07: Reserve allocation in micros (no-referral case).
    pub reserve_micros: i64,
    /// ULO basis points used.
    pub ulo_bps: u32,
    /// UNO basis points used.
    pub uno_bps: u32,
    /// Referral basis points used.
    pub referral_bps: u32,
    /// Remainder assigned to UNO (from rounding).
    pub remainder_micros: i64,
}

impl Allocation {
    /// Verify that the allocation reconciles exactly.
    ///
    /// R3-07: Includes reserve_micros in verification for no-referral cases.
    pub fn verify(&self) -> bool {
        self.ulo_micros + self.uno_micros + self.referral_micros + self.reserve_micros == self.pool_micros
    }

    /// Convert ULO allocation to dollars.
    pub fn ulo_dollars(&self) -> f64 {
        self.ulo_micros as f64 / 1_000_000.0
    }

    /// Convert UNO allocation to dollars.
    pub fn uno_dollars(&self) -> f64 {
        self.uno_micros as f64 / 1_000_000.0
    }

    /// Convert referral allocation to dollars.
    pub fn referral_dollars(&self) -> f64 {
        self.referral_micros as f64 / 1_000_000.0
    }

    /// Convert pool to dollars.
    pub fn pool_dollars(&self) -> f64 {
        self.pool_micros as f64 / 1_000_000.0
    }
}

/// Agreement version with associated revenue split.
///
/// Stores the canonical split configuration for a given agreement version.
/// Once created, agreement versions are immutable.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgreementVersion {
    /// Unique version number (monotonically increasing).
    pub version: i32,
    /// ULO share in basis points.
    pub ulo_bps: u32,
    /// UNO share in basis points.
    pub uno_bps: u32,
    /// Referral share in basis points.
    pub referral_bps: u32,
    /// Human-readable description.
    pub description: Option<String>,
    /// When this version was created.
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// Whether this is the active version for new attributions.
    pub is_active: bool,
}

impl AgreementVersion {
    /// Get the revenue split for this version.
    pub fn split(&self) -> RevenueSplit {
        RevenueSplit {
            ulo_bps: self.ulo_bps,
            uno_bps: self.uno_bps,
            referral_bps: self.referral_bps,
            agreement_version: Some(self.version),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_split_is_50_40_10() {
        let split = RevenueSplit::default();
        assert_eq!(split.ulo_bps, 5000);
        assert_eq!(split.uno_bps, 4000);
        assert_eq!(split.referral_bps, 1000);
        assert!(split.validate().is_ok());
    }

    #[test]
    fn test_split_validation_success() {
        let split = RevenueSplit::new(6000, 3000, 1000).unwrap();
        assert_eq!(split.ulo_bps, 6000);
        assert_eq!(split.uno_bps, 3000);
        assert_eq!(split.referral_bps, 1000);
    }

    #[test]
    fn test_split_validation_failure() {
        let result = RevenueSplit::new(5000, 5000, 1000);
        assert!(matches!(result, Err(RevenueSplitError::InvalidSum(11000))));
    }

    #[test]
    fn test_allocation_exact_division() {
        let split = RevenueSplit::default();
        let alloc = split.allocate(1_000_000).unwrap();

        assert_eq!(alloc.ulo_micros, 500_000);   // 50%
        assert_eq!(alloc.uno_micros, 400_000);   // 40%
        assert_eq!(alloc.referral_micros, 100_000); // 10%
        assert_eq!(alloc.reserve_micros, 0);     // R3-07: No reserve with referral
        assert_eq!(alloc.remainder_micros, 0);
        assert!(alloc.verify());
    }

    #[test]
    fn test_allocation_without_referral() {
        // R3-07: When no referral, referral share goes to reserve, not UNO
        let split = RevenueSplit::default();
        let alloc = split.allocate_without_referral(1_000_000).unwrap();

        assert_eq!(alloc.ulo_micros, 500_000);     // 50% - unchanged
        assert_eq!(alloc.uno_micros, 400_000);     // 40% - unchanged, does NOT get referral share
        assert_eq!(alloc.referral_micros, 0);       // No referral
        assert_eq!(alloc.reserve_micros, 100_000); // R3-07: 10% goes to reserve
        assert_eq!(alloc.remainder_micros, 0);
        assert!(alloc.verify());
    }

    #[test]
    fn test_allocation_with_remainder() {
        let split = RevenueSplit::default();
        // 999 doesn't divide evenly by basis points
        let alloc = split.allocate(999).unwrap();

        // Verify reconciliation
        assert!(alloc.verify());
        assert_eq!(alloc.ulo_micros + alloc.uno_micros + alloc.referral_micros, 999);

        // Check that remainder went to UNO
        // 999 * 5000 / 10000 = 499 (ulo)
        // 999 * 1000 / 10000 = 99 (referral)
        // 999 * 4000 / 10000 = 399 (uno calculated)
        // Remainder: 999 - 499 - 99 - 399 = 2
        // uno_final = 399 + 2 = 401
        assert_eq!(alloc.ulo_micros, 499);
        assert_eq!(alloc.referral_micros, 99);
        assert_eq!(alloc.uno_micros, 401);
        assert_eq!(alloc.remainder_micros, 2);
    }

    #[test]
    fn test_allocation_large_amount() {
        let split = RevenueSplit::default();
        // $1,000,000 in micros
        let alloc = split.allocate(1_000_000_000_000).unwrap();

        assert_eq!(alloc.ulo_micros, 500_000_000_000);
        assert_eq!(alloc.uno_micros, 400_000_000_000);
        assert_eq!(alloc.referral_micros, 100_000_000_000);
        assert_eq!(alloc.reserve_micros, 0);
        assert!(alloc.verify());
    }

    #[test]
    fn test_allocation_negative_pool_fails() {
        let split = RevenueSplit::default();
        let result = split.allocate(-100);
        assert!(matches!(result, Err(RevenueSplitError::InvalidPoolAmount(-100))));
    }

    #[test]
    fn test_allocation_zero_pool() {
        let split = RevenueSplit::default();
        let alloc = split.allocate(0).unwrap();

        assert_eq!(alloc.ulo_micros, 0);
        assert_eq!(alloc.uno_micros, 0);
        assert_eq!(alloc.referral_micros, 0);
        assert_eq!(alloc.reserve_micros, 0);
        assert!(alloc.verify());
    }

    #[test]
    fn test_without_referral_legacy() {
        // Legacy without_referral creates a 60/40/0 split (redistributes to UNO)
        // This is kept for backwards compatibility but should NOT be used for new code
        let split = RevenueSplit::without_referral(6000).unwrap();
        assert_eq!(split.ulo_bps, 6000);
        assert_eq!(split.uno_bps, 4000);
        assert_eq!(split.referral_bps, 0);
        assert!(split.validate().is_ok());

        let alloc = split.allocate(1_000_000).unwrap();
        assert_eq!(alloc.ulo_micros, 600_000);
        assert_eq!(alloc.uno_micros, 400_000);
        assert_eq!(alloc.referral_micros, 0);
        assert_eq!(alloc.reserve_micros, 0);
    }

    #[test]
    fn test_r3_07_no_referral_goes_to_reserve() {
        // R3-07: Use allocate_without_referral for 50/40/10 with no referral
        // Referral share goes to RESERVE, not UNO
        let split = RevenueSplit::default();
        let alloc = split.allocate_without_referral(1_000_000).unwrap();

        // ULO and UNO get their standard 50% and 40%
        assert_eq!(alloc.ulo_micros, 500_000);
        assert_eq!(alloc.uno_micros, 400_000);
        // Referral is 0 (no referral)
        assert_eq!(alloc.referral_micros, 0);
        // Reserve gets the 10% that would have gone to referral
        assert_eq!(alloc.reserve_micros, 100_000);
        assert!(alloc.verify());
    }

    #[test]
    fn test_display() {
        let split = RevenueSplit::default();
        assert_eq!(split.display(), "50/40/10");

        let custom = RevenueSplit::new(6000, 3000, 1000).unwrap();
        assert_eq!(custom.display(), "60/30/10");
    }

    #[test]
    fn test_percentages() {
        let split = RevenueSplit::default();
        assert!((split.ulo_percentage() - 50.0).abs() < 0.001);
        assert!((split.uno_percentage() - 40.0).abs() < 0.001);
        assert!((split.referral_percentage() - 10.0).abs() < 0.001);
    }

    #[test]
    fn test_dollar_conversions() {
        let split = RevenueSplit::default();
        let alloc = split.allocate(1_000_000).unwrap();

        assert!((alloc.ulo_dollars() - 0.50).abs() < 0.001);
        assert!((alloc.uno_dollars() - 0.40).abs() < 0.001);
        assert!((alloc.referral_dollars() - 0.10).abs() < 0.001);
        assert!((alloc.pool_dollars() - 1.00).abs() < 0.001);
    }

    #[test]
    fn test_revenue_identity() {
        // Test that ulo + uno + referral == pool for various amounts
        let split = RevenueSplit::default();

        for amount in [1, 7, 13, 99, 100, 999, 1000, 12345, 1_000_000, 999_999_999] {
            let alloc = split.allocate(amount).unwrap();
            assert!(
                alloc.verify(),
                "Revenue identity failed for amount {}: {} + {} + {} != {}",
                amount, alloc.ulo_micros, alloc.uno_micros, alloc.referral_micros, amount
            );
        }
    }

    #[test]
    fn test_with_version() {
        let split = RevenueSplit::with_version(5000, 4000, 1000, 2).unwrap();
        assert_eq!(split.agreement_version, Some(2));
    }
}
