//! Forecast engine service
//!
//! This module implements the core forecast simulation algorithm, ported from
//! the JavaScript HTML calculator to typed, tested Rust.
//!
//! # Algorithm Overview
//!
//! The simulation models weekly financial projections for a license portfolio:
//!
//! 1. **Device deployment**: Each week, devices churn and new ones are added
//! 2. **Cohort tracking**: Devices grouped by activation date for credit renewal
//! 3. **Task rewards**: Each enabled task contributes to the reward pool
//! 4. **Revenue split**: Pool divided as 50% ULO + 40% UNO + 10% Referral
//! 5. **Cost accounting**: Credits, support, acquisition, coordination, overhead
//! 6. **Cash flow**: Settlement lag between earned and received revenue
//!
//! # Credit Renewal
//!
//! Cohorts renew their 30-day credit blocks on anniversary dates.
//! Failed trials consume one credit block and receive half-day support.
//!
//! # Revenue Identity
//!
//! For every week: `ulo + uno + referral == pool`

use crate::models::{
    ForecastConfig, ForecastResult, ForecastRow, ForecastTask,
    ForecastValidationError, RateBasis, MAX_HORIZON_WEEKS,
};

/// Validate forecast configuration and tasks
///
/// Implements 22+ validation rules matching the JavaScript calculator.
pub fn validate(
    config: &ForecastConfig,
    tasks: &[ForecastTask],
) -> Result<(), Vec<ForecastValidationError>> {
    let mut errors = Vec::new();

    // Positive checks
    let positive_checks: [(&str, f64); 4] = [
        ("weeks", config.weeks as f64),
        ("capacity", config.capacity as f64),
        ("newNet", config.new_net as f64),
        ("amortWeeks", config.amort_weeks as f64),
    ];
    for (field, value) in positive_checks {
        if value <= 0.0 {
            errors.push(ForecastValidationError::MustBePositive(field.to_string()));
        }
    }

    // FC-04: Prevent division by zero in capital allocation
    if config.capital > 0.0 && config.amort_weeks == 0 {
        errors.push(ForecastValidationError::AmortWeeksZero);
    }

    // Maximum horizon
    if config.weeks > MAX_HORIZON_WEEKS {
        errors.push(ForecastValidationError::MaxHorizonExceeded(config.weeks));
    }

    // Range [0, 1] checks
    let range_checks: [(&str, f64); 9] = [
        ("churn", config.churn),
        ("tax", config.tax),
        ("fee", config.fee),
        ("android", config.android),
        ("ios", config.ios),
        ("windows", config.windows),
        ("uno", config.uno),
        ("ulo", config.ulo),
        ("referral", config.referral),
    ];
    for (field, value) in range_checks {
        if value < 0.0 || value > 1.0 {
            errors.push(ForecastValidationError::OutOfRange(field.to_string()));
        }
    }

    // Success rate in (0, 1]
    if config.success <= 0.0 || config.success > 1.0 {
        errors.push(ForecastValidationError::InvalidSuccessRate);
    }

    // Opening cannot exceed capacity
    if config.opening > config.capacity {
        errors.push(ForecastValidationError::OpeningExceedsCapacity);
    }

    // Device mix must sum to 1
    if (config.android + config.ios + config.windows - 1.0).abs() > 1e-8 {
        errors.push(ForecastValidationError::DeviceMixInvalid);
    }

    // Revenue shares must sum to 1
    if (config.uno + config.ulo + config.referral - 1.0).abs() > 1e-8 {
        errors.push(ForecastValidationError::SharesInvalid);
    }

    // FC-03: Reject upUsd = 0 to prevent silent zeroing
    // Only error if there are enabled tasks that would be affected
    let has_enabled_tasks = tasks.iter().any(|t| t.enabled);
    if config.up_usd == 0.0 && has_enabled_tasks {
        errors.push(ForecastValidationError::UpUsdZero);
    }

    // Non-negative checks
    let non_negative_checks: [(&str, f64); 9] = [
        ("credit", config.credit),
        ("support", config.support),
        ("acquisition", config.acquisition),
        ("coordination", config.coordination),
        ("overhead", config.overhead),
        ("setup", config.setup),
        ("capital", config.capital),
        ("openingCash", config.opening_cash),
        ("upUsd", config.up_usd),
    ];
    for (field, value) in non_negative_checks {
        if value < 0.0 {
            errors.push(ForecastValidationError::Negative(field.to_string()));
        }
    }

    // Task validation
    let enabled_tasks: Vec<_> = tasks.iter().filter(|t| t.enabled).collect();
    let has_illustrative = enabled_tasks.iter().any(|t| t.illustrative);
    let has_named = enabled_tasks.iter().any(|t| !t.illustrative);

    // Cannot mix illustrative and named tasks
    if has_illustrative && has_named {
        errors.push(ForecastValidationError::IllustrativeMixed);
    }

    // Validate each task
    for task in tasks {
        // Numeric and non-negative checks
        let task_checks: [(&str, f64); 5] = [
            ("rate", task.rate),
            ("eligible", task.eligible),
            ("activity", task.activity),
            ("cap", task.cap),
            ("extra", task.extra),
        ];
        for (field, value) in task_checks {
            if !value.is_finite() || value < 0.0 {
                errors.push(ForecastValidationError::TaskInvalid {
                    task: task.name.clone(),
                    field: field.to_string(),
                });
            }
        }

        // Factors cannot exceed 1
        if task.eligible > 1.0 {
            errors.push(ForecastValidationError::TaskFactorExceeds {
                task: task.name.clone(),
                field: "eligible".to_string(),
            });
        }
        if task.activity > 1.0 {
            errors.push(ForecastValidationError::TaskFactorExceeds {
                task: task.name.clone(),
                field: "activity".to_string(),
            });
        }

        // End must be >= start
        if task.end < task.start {
            errors.push(ForecastValidationError::TaskEndBeforeStart(task.name.clone()));
        }

        // Enabled tasks must have at least one device type
        if task.enabled && !task.android && !task.ios && !task.windows {
            errors.push(ForecastValidationError::TaskNoDevices(task.name.clone()));
        }

        // ULO basis requires positive ULO share
        if task.basis == RateBasis::Ulo && config.ulo == 0.0 {
            errors.push(ForecastValidationError::UloBasisRequiresShare(task.name.clone()));
        }
    }

    if errors.is_empty() {
        Ok(())
    } else {
        // Deduplicate errors
        errors.sort_by(|a, b| format!("{}", a).cmp(&format!("{}", b)));
        errors.dedup();
        Err(errors)
    }
}

/// A device cohort tracking activation date and count
#[derive(Debug, Clone)]
struct Cohort {
    /// Day of activation (0-based from simulation start)
    born: u32,
    /// Current device count (fractional due to churn)
    n: f64,
}

/// Run the forecast simulation
///
/// Returns weekly projections for the configured horizon.
///
/// # Algorithm
///
/// For each week:
/// 1. Apply churn to existing cohorts
/// 2. Calculate net additions (capped at capacity)
/// 3. Process each day (7 days per week):
///    - Check for 30-day credit renewal anniversaries
///    - Add new devices (joining mid-day)
///    - Calculate exposure for each enabled task
///    - Accumulate reward pool and costs
/// 4. Split revenue (ULO/UNO/Referral)
/// 5. Calculate profit and cash flow
pub fn simulate(
    config: &ForecastConfig,
    tasks: &[ForecastTask],
) -> Result<ForecastResult, Vec<ForecastValidationError>> {
    // Validate first
    validate(config, tasks)?;

    // Initialize cohorts
    let mut cohorts: Vec<Cohort> = if config.opening > 0 {
        vec![Cohort { born: 0, n: config.opening as f64 }]
    } else {
        Vec::new()
    };

    let mut rows: Vec<ForecastRow> = Vec::with_capacity(config.weeks as usize);
    let mut cumulative_profit = -(config.setup);
    let mut cash = config.opening_cash - config.setup;
    let mut cumulative_cash = -(config.setup);
    let mut peak = -cumulative_cash; // Track peak funding requirement

    for w in 1..=config.weeks {
        // Opening count
        let opening: f64 = cohorts.iter().map(|c| c.n).sum();

        // Apply churn
        let loss = opening * config.churn;
        for cohort in &mut cohorts {
            cohort.n *= 1.0 - config.churn;
        }

        // Calculate net additions (capped at remaining capacity)
        let remaining_capacity = (config.capacity as f64) - opening;
        let net = (config.new_net as f64).min(remaining_capacity.max(0.0));
        let additions = net + loss;

        // Calculate attempts (round up)
        let attempts = ((additions - 1e-9) / config.success).ceil();
        let failures = attempts - additions;

        // Initialize row
        let mut row = ForecastRow {
            week: w,
            opening,
            churn: loss,
            additions,
            attempts,
            closing: opening + net,
            device_days: 0.0,
            pool: 0.0,
            ulo: 0.0,
            referral: 0.0,
            uno: 0.0,
            credits: 0.0,
            support: 0.0,
            task_costs: 0.0,
            acquisition: if net > 1e-9 { config.acquisition } else { config.maintenance_acquisition },
            coordination: if net > 1e-9 { config.coordination } else { config.maintenance_coordination },
            overhead: config.overhead,
            fees: 0.0,
            // FC-04: Use floating-point division to prevent truncation
            capital_charge: if w <= config.amort_weeks && config.amort_weeks > 0 {
                config.capital / (config.amort_weeks as f64)
            } else {
                0.0
            },
            tax: 0.0,
            operating_costs: 0.0,
            pretax: 0.0,
            profit: 0.0,
            cumulative_profit: 0.0,
            receipts: 0.0,
            cash_costs: 0.0,
            cash_flow: 0.0,
            cash: 0.0,
            cumulative_cash: 0.0,
            required_funding: 0.0,
        };

        // Week 1: charge credits for opening survivors
        if w == 1 && config.opening > 0 {
            row.credits += (config.opening as f64 - loss) * config.credit;
        }

        // Process each day
        for d in ((w - 1) * 7)..(w * 7) {
            // Check for 30-day credit renewal anniversaries
            for cohort in &cohorts {
                if d > cohort.born && (d - cohort.born) % 30 == 0 {
                    row.credits += cohort.n * config.credit;
                }
            }

            // Count existing devices
            let old: f64 = cohorts.iter().map(|c| c.n).sum();
            let new_n = additions / 7.0;

            // Exposure: existing devices full day + new devices half day
            let exposure = old + new_n * 0.5;
            row.device_days += exposure;

            // Credits for new activation attempts
            row.credits += (attempts / 7.0) * config.credit;

            // Support: exposure + failures/14 (half-day support for failures)
            // FC-06: Document failures/14 support term
            row.support += (exposure + failures / 14.0) * config.support / 30.0;

            // Process enabled tasks
            for task in tasks {
                if !task.enabled || w < task.start || w > task.end {
                    continue;
                }

                let device_mix = task.device_mix(config);
                let eligible = exposure * device_mix * task.eligible * task.activity;
                let rate = task.effective_rate(config);

                let pool_contribution = if task.cap > 0.0 {
                    (eligible * rate).min(task.cap * config.up_usd)
                } else {
                    eligible * rate
                };

                row.pool += pool_contribution;
                row.task_costs += eligible * task.extra;
            }

            // Add new cohort for this day
            cohorts.push(Cohort { born: d, n: new_n });
        }

        // Revenue split (FC-07: maintains revenue identity)
        row.uno = row.pool * config.uno;
        row.ulo = row.pool * config.ulo;
        row.referral = row.pool * config.referral;

        // Fees
        row.fees = row.uno * config.fee;

        // Operating costs
        row.operating_costs = row.credits + row.support + row.task_costs
            + row.acquisition + row.coordination + row.overhead + row.fees;

        // Profit calculation
        row.pretax = row.uno - row.operating_costs - row.capital_charge;
        row.tax = (row.pretax.max(0.0)) * config.tax;
        row.profit = row.pretax - row.tax;

        cumulative_profit += row.profit;
        row.cumulative_profit = cumulative_profit;

        // Cash flow with settlement lag
        let paid_index = if w > config.lag { (w - 1 - config.lag) as usize } else { usize::MAX };
        let source = if config.lag == 0 {
            Some(&row)
        } else if paid_index < rows.len() {
            Some(&rows[paid_index])
        } else {
            None
        };

        row.receipts = source.map(|s| s.uno - s.fees).unwrap_or(0.0);
        row.cash_costs = row.operating_costs - row.fees + row.tax;
        row.cash_flow = row.receipts - row.cash_costs;

        cash += row.cash_flow;
        cumulative_cash += row.cash_flow;

        row.cash = cash;
        row.cumulative_cash = cumulative_cash;

        // Track peak funding requirement
        if -cumulative_cash > peak {
            peak = -cumulative_cash;
        }
        row.required_funding = peak;

        rows.push(row);
    }

    // Find milestone weeks
    let first_positive_week = rows.iter()
        .find(|r| r.profit > 0.0)
        .map(|r| r.week);

    let profit_payback = rows.iter()
        .find(|r| r.cumulative_profit >= 0.0)
        .map(|r| r.week);

    let cash_payback = rows.iter()
        .find(|r| r.cumulative_cash >= 0.0)
        .map(|r| r.week);

    Ok(ForecastResult {
        rows,
        required_funding: peak,
        first_positive_week,
        profit_payback,
        cash_payback,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::ForecastTask;

    #[test]
    fn test_validate_default_config() {
        let config = ForecastConfig::default();
        let tasks = ForecastTask::default_tasks();
        assert!(validate(&config, &tasks).is_ok());
    }

    #[test]
    fn test_validate_rejects_zero_weeks() {
        let mut config = ForecastConfig::default();
        config.weeks = 0;
        let tasks = ForecastTask::default_tasks();
        let result = validate(&config, &tasks);
        assert!(result.is_err());
        assert!(result.unwrap_err().iter().any(|e| matches!(e, ForecastValidationError::MustBePositive(_))));
    }

    #[test]
    fn test_validate_rejects_excessive_weeks() {
        let mut config = ForecastConfig::default();
        config.weeks = 300;
        let tasks = ForecastTask::default_tasks();
        let result = validate(&config, &tasks);
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_rejects_invalid_success_rate() {
        let mut config = ForecastConfig::default();
        config.success = 0.0;
        let tasks = ForecastTask::default_tasks();
        let result = validate(&config, &tasks);
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_rejects_device_mix_not_one() {
        let mut config = ForecastConfig::default();
        config.android = 0.5;
        config.ios = 0.3;
        config.windows = 0.1; // Sum = 0.9
        let tasks = ForecastTask::default_tasks();
        let result = validate(&config, &tasks);
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_rejects_shares_not_one() {
        let mut config = ForecastConfig::default();
        config.uno = 0.5;
        config.ulo = 0.5;
        config.referral = 0.1; // Sum = 1.1
        let tasks = ForecastTask::default_tasks();
        let result = validate(&config, &tasks);
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_rejects_mixed_tasks() {
        let config = ForecastConfig::default();
        let mut tasks = ForecastTask::default_tasks();
        tasks[0].enabled = true; // Illustrative
        tasks[1].enabled = true; // Named
        let result = validate(&config, &tasks);
        assert!(result.is_err());
        assert!(result.unwrap_err().iter().any(|e| matches!(e, ForecastValidationError::IllustrativeMixed)));
    }

    #[test]
    fn test_validate_fc03_rejects_zero_up_usd() {
        let mut config = ForecastConfig::default();
        config.up_usd = 0.0;
        let tasks = ForecastTask::default_tasks(); // Has enabled task
        let result = validate(&config, &tasks);
        assert!(result.is_err());
        assert!(result.unwrap_err().iter().any(|e| matches!(e, ForecastValidationError::UpUsdZero)));
    }

    #[test]
    fn test_validate_fc04_rejects_zero_amort_weeks_with_capital() {
        let mut config = ForecastConfig::default();
        config.capital = 10000.0;
        config.amort_weeks = 0;
        let tasks = ForecastTask::default_tasks();
        let result = validate(&config, &tasks);
        assert!(result.is_err());
        assert!(result.unwrap_err().iter().any(|e| matches!(e, ForecastValidationError::AmortWeeksZero)));
    }

    #[test]
    fn test_simulate_default_scenario() {
        let config = ForecastConfig::default();
        let tasks = ForecastTask::default_tasks();
        let result = simulate(&config, &tasks).unwrap();

        assert_eq!(result.rows.len(), 52);
        assert!(result.required_funding > 0.0);
    }

    #[test]
    fn test_simulate_revenue_identity_fc07() {
        let config = ForecastConfig::default();
        let tasks = ForecastTask::default_tasks();
        let result = simulate(&config, &tasks).unwrap();

        // FC-07: Assert revenue identity for every row
        for row in &result.rows {
            let sum = row.ulo + row.uno + row.referral;
            assert!(
                (row.pool - sum).abs() < 1e-8,
                "Revenue identity violated in week {}: pool={}, sum={}",
                row.week, row.pool, sum
            );
        }

        assert!(result.all_revenue_identities_hold());
    }

    #[test]
    fn test_simulate_closing_count_grows() {
        let config = ForecastConfig::default();
        let tasks = ForecastTask::default_tasks();
        let result = simulate(&config, &tasks).unwrap();

        // Should grow from 0 to capacity
        assert_eq!(result.rows[0].opening, 0.0);
        assert!(result.rows.last().unwrap().closing > 2000.0);
    }

    #[test]
    fn test_simulate_with_opening_inventory() {
        let mut config = ForecastConfig::default();
        config.opening = 500;
        let tasks = ForecastTask::default_tasks();
        let result = simulate(&config, &tasks).unwrap();

        assert_eq!(result.rows[0].opening, 500.0);
    }

    #[test]
    fn test_simulate_settlement_lag() {
        let mut config = ForecastConfig::default();
        config.lag = 2;
        let tasks = ForecastTask::default_tasks();
        let result = simulate(&config, &tasks).unwrap();

        // First two weeks should have zero receipts due to lag
        assert_eq!(result.rows[0].receipts, 0.0);
        assert_eq!(result.rows[1].receipts, 0.0);
        // Third week should have receipts from week 1
        assert!(result.rows[2].receipts > 0.0);
    }

    #[test]
    fn test_simulate_zero_lag() {
        let mut config = ForecastConfig::default();
        config.lag = 0;
        let tasks = ForecastTask::default_tasks();
        let result = simulate(&config, &tasks).unwrap();

        // With zero lag, receipts should equal uno - fees in same week
        for row in &result.rows {
            let expected = row.uno - row.fees;
            assert!(
                (row.receipts - expected).abs() < 1e-8,
                "Week {}: receipts={}, expected={}",
                row.week, row.receipts, expected
            );
        }
    }

    #[test]
    fn test_simulate_no_enabled_tasks() {
        let config = ForecastConfig::default();
        let mut tasks = ForecastTask::default_tasks();
        for task in &mut tasks {
            task.enabled = false;
        }
        let result = simulate(&config, &tasks).unwrap();

        // Pool should be zero with no enabled tasks
        for row in &result.rows {
            assert_eq!(row.pool, 0.0);
        }
    }

    #[test]
    fn test_simulate_task_with_cap() {
        let config = ForecastConfig::default();
        let tasks = vec![ForecastTask {
            name: "Capped task".to_string(),
            enabled: true,
            android: true,
            ios: false,
            windows: false,
            rate: 1.0,
            basis: RateBasis::Pool,
            eligible: 1.0,
            activity: 1.0,
            start: 1,
            end: 52,
            cap: 10.0, // $10 daily cap
            extra: 0.0,
            illustrative: true,
        }];

        let result = simulate(&config, &tasks).unwrap();

        // Pool should be capped
        for row in &result.rows {
            // Daily cap is 10 * 7 days * up_usd
            let max_weekly = 10.0 * 7.0 * config.up_usd;
            assert!(row.pool <= max_weekly + 1e-6);
        }
    }

    #[test]
    fn test_simulate_fc04_capital_charge_no_truncation() {
        let mut config = ForecastConfig::default();
        config.capital = 1000.0;
        config.amort_weeks = 3; // Would truncate to 333 in integer division
        config.weeks = 5;

        let tasks = ForecastTask::default_tasks();
        let result = simulate(&config, &tasks).unwrap();

        // Check that capital charge uses floating-point division
        let expected_charge = 1000.0 / 3.0; // ~333.33
        for row in &result.rows[..3] {
            assert!(
                (row.capital_charge - expected_charge).abs() < 0.01,
                "Capital charge should be ~{}, got {}",
                expected_charge, row.capital_charge
            );
        }

        // After amort_weeks, capital charge should be zero
        assert_eq!(result.rows[3].capital_charge, 0.0);
        assert_eq!(result.rows[4].capital_charge, 0.0);
    }

    #[test]
    fn test_simulate_fc05_credit_renewal_sawtooth() {
        let mut config = ForecastConfig::default();
        config.weeks = 8; // ~56 days, past first 30-day renewal
        config.opening = 100;
        config.new_net = 1; // Minimal growth to satisfy validation
        config.churn = 0.0; // No churn
        config.capacity = 200; // Ensure room for growth

        let tasks = ForecastTask::default_tasks();
        let result = simulate(&config, &tasks).unwrap();

        // Week 1: initial credits for opening inventory
        assert!(result.rows[0].credits > 0.0);

        // Week 5 (days 28-34) should have renewal spike for day 30
        // (opening cohort born on day 0 renews on day 30)
        let week5_credits = result.rows[4].credits;

        // Week 5 should have significant credits due to renewal of 100 opening devices
        // The opening cohort (100 devices) renews on day 30 which falls in week 5
        assert!(
            week5_credits > config.credit * 50.0,
            "Expected credit renewal around week 5, got credits={}", week5_credits
        );
    }

    #[test]
    fn test_simulate_fc06_failures_support_term() {
        let mut config = ForecastConfig::default();
        config.weeks = 1;
        config.opening = 0;
        config.new_net = 100;
        config.success = 0.5; // 50% success = lots of failures
        config.support = 1.0;

        let tasks = ForecastTask::default_tasks();
        let result = simulate(&config, &tasks).unwrap();

        // With 50% success, attempts = 200, failures = 100
        // Support should include failures/14 term
        let row = &result.rows[0];

        // failures = 200 - 100 = 100
        // Support includes: exposure * support/30 + (failures/14) * support/30
        assert!(row.support > 0.0);
    }

    #[test]
    fn test_csv_export() {
        let config = ForecastConfig::default();
        let tasks = ForecastTask::default_tasks();
        let result = simulate(&config, &tasks).unwrap();

        let csv = result.to_csv();
        let lines: Vec<_> = csv.lines().collect();

        // Header + 52 data rows
        assert_eq!(lines.len(), 53);
        assert!(lines[0].starts_with("week,"));
        assert!(lines[1].starts_with("1,"));
    }

    #[test]
    fn test_downside_reference_upside_scenarios() {
        use crate::models::ForecastScenario;

        let scenarios = ForecastScenario::default_scenarios();

        for scenario in scenarios {
            let result = simulate(&scenario.config, &scenario.tasks);
            assert!(result.is_ok(), "Scenario '{}' failed validation", scenario.name);

            let result = result.unwrap();
            assert!(result.all_revenue_identities_hold());
        }
    }
}
