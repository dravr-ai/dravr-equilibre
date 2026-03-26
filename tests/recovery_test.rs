// ABOUTME: Tests for recovery metrics models including HRV, stress, and readiness scoring
// ABOUTME: Validates StoredRecoveryMetrics construction, boundary values, and JSON roundtrips
//
// SPDX-License-Identifier: Apache-2.0
// Copyright (c) 2026 dravr.ai

use chrono::{NaiveDate, Utc};
use dravr_equilibre::recovery::StoredRecoveryMetrics;

/// Helper to create a fully populated recovery metrics instance.
fn full_recovery_metrics() -> StoredRecoveryMetrics {
    StoredRecoveryMetrics {
        id: "rec-full".to_owned(),
        user_id: "user-001".to_owned(),
        data_source_id: "ds-garmin".to_owned(),
        date: NaiveDate::from_ymd_opt(2026, 3, 24).expect("valid calendar date"),
        recovery_score: Some(78),
        readiness_score: Some(82),
        hrv_ms: Some(45.0),
        hrv_rmssd: Some(42.0),
        resting_heart_rate: Some(52),
        stress_score: Some(25),
        body_battery: Some(65),
        spo2: Some(97.5),
        respiratory_rate: Some(14.5),
        skin_temp_deviation: Some(0.3),
        source_name: "Garmin".to_owned(),
        recorded_at: Utc::now(),
    }
}

#[test]
fn test_stored_recovery_metrics_all_fields() {
    let metrics = full_recovery_metrics();

    assert_eq!(metrics.id, "rec-full");
    assert_eq!(metrics.user_id, "user-001");
    assert_eq!(metrics.data_source_id, "ds-garmin");
    assert_eq!(metrics.date, NaiveDate::from_ymd_opt(2026, 3, 24).expect("valid date"));
    assert_eq!(metrics.recovery_score, Some(78));
    assert_eq!(metrics.readiness_score, Some(82));
    assert_eq!(metrics.hrv_ms, Some(45.0));
    assert_eq!(metrics.hrv_rmssd, Some(42.0));
    assert_eq!(metrics.resting_heart_rate, Some(52));
    assert_eq!(metrics.stress_score, Some(25));
    assert_eq!(metrics.body_battery, Some(65));
    assert_eq!(metrics.spo2, Some(97.5));
    assert_eq!(metrics.respiratory_rate, Some(14.5));
    assert_eq!(metrics.skin_temp_deviation, Some(0.3));
    assert_eq!(metrics.source_name, "Garmin");
}

#[test]
fn test_stored_recovery_metrics_all_optional_fields_none() {
    let metrics = StoredRecoveryMetrics {
        id: "rec-min".to_owned(),
        user_id: "user-002".to_owned(),
        data_source_id: "ds-apple".to_owned(),
        date: NaiveDate::from_ymd_opt(2026, 3, 25).expect("valid date"),
        recovery_score: None,
        readiness_score: None,
        hrv_ms: None,
        hrv_rmssd: None,
        resting_heart_rate: None,
        stress_score: None,
        body_battery: None,
        spo2: None,
        respiratory_rate: None,
        skin_temp_deviation: None,
        source_name: "Apple".to_owned(),
        recorded_at: Utc::now(),
    };

    assert!(metrics.recovery_score.is_none());
    assert!(metrics.readiness_score.is_none());
    assert!(metrics.hrv_ms.is_none());
    assert!(metrics.hrv_rmssd.is_none());
    assert!(metrics.resting_heart_rate.is_none());
    assert!(metrics.stress_score.is_none());
    assert!(metrics.body_battery.is_none());
    assert!(metrics.spo2.is_none());
    assert!(metrics.respiratory_rate.is_none());
    assert!(metrics.skin_temp_deviation.is_none());
}

#[test]
fn test_recovery_score_range_zero() {
    let mut metrics = full_recovery_metrics();
    metrics.recovery_score = Some(0);
    assert_eq!(metrics.recovery_score, Some(0));
}

#[test]
fn test_recovery_score_range_hundred() {
    let mut metrics = full_recovery_metrics();
    metrics.recovery_score = Some(100);
    assert_eq!(metrics.recovery_score, Some(100));
}

#[test]
fn test_readiness_score_range_zero() {
    let mut metrics = full_recovery_metrics();
    metrics.readiness_score = Some(0);
    assert_eq!(metrics.readiness_score, Some(0));
}

#[test]
fn test_readiness_score_range_hundred() {
    let mut metrics = full_recovery_metrics();
    metrics.readiness_score = Some(100);
    assert_eq!(metrics.readiness_score, Some(100));
}

#[test]
fn test_stress_level_zero() {
    let mut metrics = full_recovery_metrics();
    metrics.stress_score = Some(0);
    assert_eq!(metrics.stress_score, Some(0));
}

#[test]
fn test_stress_level_hundred() {
    let mut metrics = full_recovery_metrics();
    metrics.stress_score = Some(100);
    assert_eq!(metrics.stress_score, Some(100));
}

#[test]
fn test_resting_heart_rate_low_athlete() {
    let mut metrics = full_recovery_metrics();
    metrics.resting_heart_rate = Some(40);
    assert_eq!(metrics.resting_heart_rate, Some(40));
}

#[test]
fn test_resting_heart_rate_high_end() {
    let mut metrics = full_recovery_metrics();
    metrics.resting_heart_rate = Some(100);
    assert_eq!(metrics.resting_heart_rate, Some(100));
}

#[test]
fn test_skin_temp_deviation_negative() {
    let mut metrics = full_recovery_metrics();
    metrics.skin_temp_deviation = Some(-2.0);
    assert_eq!(metrics.skin_temp_deviation, Some(-2.0));
}

#[test]
fn test_skin_temp_deviation_positive() {
    let mut metrics = full_recovery_metrics();
    metrics.skin_temp_deviation = Some(2.0);
    assert_eq!(metrics.skin_temp_deviation, Some(2.0));
}

#[test]
fn test_json_serialization_roundtrip() {
    let metrics = full_recovery_metrics();
    let json = serde_json::to_string(&metrics)
        .expect("recovery metrics should serialize to JSON");
    let roundtripped: StoredRecoveryMetrics = serde_json::from_str(&json)
        .expect("JSON should deserialize back to StoredRecoveryMetrics");

    assert_eq!(roundtripped.id, metrics.id);
    assert_eq!(roundtripped.user_id, metrics.user_id);
    assert_eq!(roundtripped.date, metrics.date);
    assert_eq!(roundtripped.recovery_score, metrics.recovery_score);
    assert_eq!(roundtripped.readiness_score, metrics.readiness_score);
    assert_eq!(roundtripped.hrv_ms, metrics.hrv_ms);
    assert_eq!(roundtripped.resting_heart_rate, metrics.resting_heart_rate);
    assert_eq!(roundtripped.stress_score, metrics.stress_score);
    assert_eq!(roundtripped.skin_temp_deviation, metrics.skin_temp_deviation);
}

#[test]
fn test_json_optional_fields_present_when_populated() {
    let metrics = full_recovery_metrics();
    let json = serde_json::to_string(&metrics)
        .expect("recovery metrics should serialize");

    assert!(json.contains("\"recovery_score\""), "populated recovery_score should appear in JSON");
    assert!(json.contains("\"hrv_ms\""), "populated hrv_ms should appear in JSON");
    assert!(json.contains("\"skin_temp_deviation\""), "populated skin_temp_deviation should appear in JSON");
}
