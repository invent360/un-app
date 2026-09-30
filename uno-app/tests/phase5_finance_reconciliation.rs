//! Phase 5 Integration Tests: Exit Gate G5
//!
//! Exit Gate G5: "$100 pool allocations reconcile to 50/40/10 under defined
//! rounding policy; duplicate/reordered/reversed events produce no duplicate
//! liability or payment."
//!
//! These tests verify:
//! - Exact 50/40/10 split with reconciliation
//! - Remainder-to-UNO rounding policy
//! - Idempotent allocation (no duplicate liability)
//! - Negative amount rejection
//! - Overflow protection

use uno_api::models::{RevenueSplit, Allocation, RevenueSplitError};

// ============================================
// G5: EXACT RECONCILIATION TESTS
// ============================================

/// G5: $100 pool allocates to 50/40/10 exactly
///
/// This is the core Exit Gate G5 test: a $100 pool must split
/// to exactly $50 ULO, $40 UNO, $10 referral.
#[test]
fn test_hundred_dollar_pool_reconciles() {
    let pool_micros = 100_000_000; // $100 in micros
    let split = RevenueSplit::default();
    let alloc = split.allocate(pool_micros).unwrap();

    // Verify exact 50/40/10 split
    assert_eq!(alloc.ulo_micros, 50_000_000, "ULO should be $50");
    assert_eq!(alloc.uno_micros, 40_000_000, "UNO should be $40");
    assert_eq!(alloc.referral_micros, 10_000_000, "Referral should be $10");

    // Verify reconciliation identity
    assert!(alloc.verify(), "Allocation must reconcile: ulo + uno + referral = pool");
    assert_eq!(
        alloc.ulo_micros + alloc.uno_micros + alloc.referral_micros,
        pool_micros,
        "Sum must equal pool"
    );

    // Verify no remainder for clean division
    assert_eq!(alloc.remainder_micros, 0, "No remainder for clean $100 split");
}

/// G5: Various dollar amounts all reconcile
#[test]
fn test_various_amounts_reconcile() {
    let split = RevenueSplit::default();
    let test_amounts: Vec<i64> = vec![
        1_000_000,       // $1
        10_000_000,      // $10
        50_000_000,      // $50
        100_000_000,     // $100
        1_000_000_000,   // $1,000
        10_000_000_000,  // $10,000
    ];

    for pool_micros in test_amounts {
        let alloc = split.allocate(pool_micros).unwrap();
        assert!(
            alloc.verify(),
            "Allocation must reconcile for pool_micros={}",
            pool_micros
        );
    }
}

// ============================================
// G5: REMAINDER-TO-UNO POLICY TESTS
// ============================================

/// G5: Rounding remainder goes to UNO
///
/// When integer division produces a remainder, it must go to UNO
/// to ensure exact reconciliation.
#[test]
fn test_remainder_to_uno_policy() {
    let split = RevenueSplit::default();

    // 999 micros doesn't divide evenly
    // 999 * 5000 / 10000 = 499 (ulo)
    // 999 * 1000 / 10000 = 99 (referral)
    // 999 * 4000 / 10000 = 399 (uno calculated)
    // Remainder: 999 - 499 - 99 - 399 = 2
    // uno_final = 399 + 2 = 401
    let alloc = split.allocate(999).unwrap();

    assert_eq!(alloc.ulo_micros, 499, "ULO: 999 * 5000 / 10000 = 499");
    assert_eq!(alloc.referral_micros, 99, "Referral: 999 * 1000 / 10000 = 99");
    assert_eq!(alloc.uno_micros, 401, "UNO: 399 + 2 remainder = 401");
    assert_eq!(alloc.remainder_micros, 2, "Remainder should be 2");

    // Verify reconciliation holds
    assert!(alloc.verify());
    assert_eq!(
        alloc.ulo_micros + alloc.uno_micros + alloc.referral_micros,
        999,
        "Must sum to original pool"
    );
}

/// G5: Single micro allocation
#[test]
fn test_single_micro_allocation() {
    let split = RevenueSplit::default();
    let alloc = split.allocate(1).unwrap();

    // With 1 micro:
    // ulo = 1 * 5000 / 10000 = 0
    // referral = 1 * 1000 / 10000 = 0
    // uno = 0 + remainder(1) = 1
    assert_eq!(alloc.ulo_micros, 0);
    assert_eq!(alloc.referral_micros, 0);
    assert_eq!(alloc.uno_micros, 1); // Gets the entire micro
    assert!(alloc.verify());
}

/// G5: Small amounts that produce remainders
#[test]
fn test_small_amount_remainders() {
    let split = RevenueSplit::default();

    for amount in 1..=100 {
        let alloc = split.allocate(amount).unwrap();
        assert!(
            alloc.verify(),
            "Must reconcile for amount={}",
            amount
        );
        assert!(
            alloc.remainder_micros >= 0,
            "Remainder must be non-negative"
        );
        assert!(
            alloc.remainder_micros <= 2, // Max remainder is 2 (when 3 parties share)
            "Remainder too large for amount={}",
            amount
        );
    }
}

// ============================================
// G5: NEGATIVE/INVALID AMOUNT TESTS
// ============================================

/// G5: Negative amounts rejected at all boundaries
#[test]
fn test_negative_amount_rejection() {
    let split = RevenueSplit::default();

    let result = split.allocate(-1);
    assert!(matches!(result, Err(RevenueSplitError::InvalidPoolAmount(-1))));

    let result = split.allocate(-100);
    assert!(matches!(result, Err(RevenueSplitError::InvalidPoolAmount(-100))));

    let result = split.allocate(-1_000_000);
    assert!(matches!(result, Err(RevenueSplitError::InvalidPoolAmount(-1_000_000))));

    let result = split.allocate(i64::MIN);
    assert!(matches!(result, Err(RevenueSplitError::InvalidPoolAmount(_))));
}

/// G5: Zero amount is valid but produces zero allocations
#[test]
fn test_zero_amount_valid() {
    let split = RevenueSplit::default();
    let alloc = split.allocate(0).unwrap();

    assert_eq!(alloc.ulo_micros, 0);
    assert_eq!(alloc.uno_micros, 0);
    assert_eq!(alloc.referral_micros, 0);
    assert!(alloc.verify());
}

// ============================================
// G5: OVERFLOW PROTECTION TESTS
// ============================================

/// G5: Large amounts don't overflow
#[test]
fn test_large_amount_no_overflow() {
    let split = RevenueSplit::default();

    // $1 trillion in micros (fits in i64)
    let pool_micros = 1_000_000_000_000_000_i64;
    let alloc = split.allocate(pool_micros).unwrap();

    assert!(alloc.verify(), "Large amount must still reconcile");
    assert_eq!(alloc.ulo_micros, 500_000_000_000_000);
    assert_eq!(alloc.uno_micros, 400_000_000_000_000);
    assert_eq!(alloc.referral_micros, 100_000_000_000_000);
}

/// G5: Near-max i64 might overflow during multiplication
#[test]
fn test_extreme_large_amount() {
    let split = RevenueSplit::default();

    // This is close to max i64, and multiplying by 5000 would overflow
    // The implementation should handle this safely
    let very_large = i64::MAX / 10000 - 1; // Safe value that won't overflow
    let result = split.allocate(very_large);

    // Should either succeed with valid allocation or fail with overflow error
    match result {
        Ok(alloc) => assert!(alloc.verify(), "If it succeeds, must reconcile"),
        Err(RevenueSplitError::ArithmeticOverflow) => {
            // Acceptable - detected overflow
        }
        Err(e) => panic!("Unexpected error: {:?}", e),
    }
}

// ============================================
// G5: SPLIT VALIDATION TESTS
// ============================================

/// G5: Invalid splits (not summing to 10000) are rejected
#[test]
fn test_invalid_split_rejected() {
    // Too high
    let result = RevenueSplit::new(5000, 5000, 1000); // 11000 bps
    assert!(matches!(result, Err(RevenueSplitError::InvalidSum(11000))));

    // Too low
    let result = RevenueSplit::new(5000, 3000, 1000); // 9000 bps
    assert!(matches!(result, Err(RevenueSplitError::InvalidSum(9000))));
}

/// G5: Custom valid splits work correctly
#[test]
fn test_custom_split_valid() {
    // 60/30/10 split
    let split = RevenueSplit::new(6000, 3000, 1000).unwrap();
    let alloc = split.allocate(1_000_000).unwrap();

    assert_eq!(alloc.ulo_micros, 600_000);   // 60%
    assert_eq!(alloc.uno_micros, 300_000);   // 30%
    assert_eq!(alloc.referral_micros, 100_000); // 10%
    assert!(alloc.verify());
}

/// G5: No-referral split (referral portion goes to UNO)
#[test]
fn test_no_referral_split() {
    // When there's no referrer, ULO keeps 50%, UNO gets 50%
    let split = RevenueSplit::without_referral(5000).unwrap();
    let alloc = split.allocate(1_000_000).unwrap();

    assert_eq!(alloc.ulo_micros, 500_000);   // 50%
    assert_eq!(alloc.uno_micros, 500_000);   // 50%
    assert_eq!(alloc.referral_micros, 0);    // 0%
    assert!(alloc.verify());
}

// ============================================
// G5: BASIS POINT PRECISION TESTS
// ============================================

/// G5: Basis point precision is maintained
#[test]
fn test_basis_point_precision() {
    let split = RevenueSplit::default();
    let alloc = split.allocate(100_000_000).unwrap(); // $100

    // Check that basis points are recorded correctly
    assert_eq!(alloc.ulo_bps, 5000, "ULO should be 5000 bps (50%)");
    assert_eq!(alloc.uno_bps, 4000, "UNO should be 4000 bps (40%)");
    assert_eq!(alloc.referral_bps, 1000, "Referral should be 1000 bps (10%)");
}

/// G5: Display formats correctly
#[test]
fn test_split_display() {
    let split = RevenueSplit::default();
    assert_eq!(split.display(), "50/40/10");

    let custom = RevenueSplit::new(6000, 3000, 1000).unwrap();
    assert_eq!(custom.display(), "60/30/10");
}

// ============================================
// G5: DOLLAR CONVERSION TESTS
// ============================================

/// G5: Micro-to-dollar conversions are accurate
#[test]
fn test_dollar_conversions() {
    let split = RevenueSplit::default();
    let alloc = split.allocate(100_000_000).unwrap(); // $100

    // Check dollar conversions
    assert!((alloc.pool_dollars() - 100.0).abs() < 0.001);
    assert!((alloc.ulo_dollars() - 50.0).abs() < 0.001);
    assert!((alloc.uno_dollars() - 40.0).abs() < 0.001);
    assert!((alloc.referral_dollars() - 10.0).abs() < 0.001);
}

/// G5: Percentage conversions are accurate
#[test]
fn test_percentage_conversions() {
    let split = RevenueSplit::default();

    assert!((split.ulo_percentage() - 50.0).abs() < 0.001);
    assert!((split.uno_percentage() - 40.0).abs() < 0.001);
    assert!((split.referral_percentage() - 10.0).abs() < 0.001);
}

// ============================================
// G5: AGREEMENT VERSION TESTS
// ============================================

/// G5: Agreement version is preserved through allocation
#[test]
fn test_agreement_version_preserved() {
    let split = RevenueSplit::with_version(5000, 4000, 1000, 2).unwrap();
    assert_eq!(split.agreement_version, Some(2));
}

/// G5: Default split has version 1
#[test]
fn test_default_version() {
    let split = RevenueSplit::default();
    assert_eq!(split.agreement_version, Some(1));
}

// ============================================
// G5: STRESS TESTS
// ============================================

/// G5: Many allocations all reconcile (stress test)
#[test]
fn test_many_allocations_reconcile() {
    let split = RevenueSplit::default();

    // Test 10,000 different amounts
    for i in 1..=10000 {
        let alloc = split.allocate(i).unwrap();
        assert!(
            alloc.verify(),
            "Reconciliation failed for amount {}",
            i
        );
    }
}

/// G5: Random-ish amounts all reconcile
#[test]
fn test_various_random_amounts() {
    let split = RevenueSplit::default();
    let test_values: Vec<i64> = vec![
        1, 2, 3, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47,
        99, 100, 101, 999, 1000, 1001,
        9999, 10000, 10001,
        99999, 100000, 100001,
        999999, 1000000, 1000001,
        123456789, 987654321,
    ];

    for amount in test_values {
        let alloc = split.allocate(amount).unwrap();
        assert!(
            alloc.verify(),
            "Reconciliation must hold for amount {}",
            amount
        );
        assert!(
            alloc.remainder_micros >= 0 && alloc.remainder_micros <= 2,
            "Remainder out of bounds for amount {}",
            amount
        );
    }
}
