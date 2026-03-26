// ABOUTME: Edge case tests for domain models including boundary values and unusual inputs
// ABOUTME: Validates empty strings, Unicode, epoch dates, midnight-spanning sessions, and extremes
//
// SPDX-License-Identifier: Apache-2.0
// Copyright (c) 2026 dravr.ai

use chrono::{Duration, NaiveDate, TimeZone, Utc};
use dravr_equilibre::data_source::{DataSource, DeviceType};
use dravr_equilibre::health::StoredHealthMetrics;
use dravr_equilibre::recovery::StoredRecoveryMetrics;
use dravr_equilibre::sleep::StoredSleepSession;

#[test]
fn test_empty_string_ids() {
    let ds = DataSource {
        id: String::new(),
        user_id: String::new(),
        provider: String::new(),
        device_model: None,
        software_version: None,
        source: None,
        device_type: DeviceType::Unknown,
        original_source_name: None,
    };

    assert!(ds.id.is_empty());
    assert!(ds.user_id.is_empty());
    assert!(ds.provider.is_empty());
}

#[test]
fn test_very_long_provider_name() {
    let long_name = "a".repeat(1000);
    let ds = DataSource {
        id: "ds-long".to_owned(),
        user_id: "user-001".to_owned(),
        provider: long_name.clone(),
        device_model: None,
        software_version: None,
        source: None,
        device_type: DeviceType::Unknown,
        original_source_name: None,
    };

    assert_eq!(ds.provider.len(), 1000);
    assert_eq!(ds.provider, long_name);
}

#[test]
fn test_unicode_device_model() {
    let ds = DataSource {
        id: "ds-unicode".to_owned(),
        user_id: "user-001".to_owned(),
        provider: "garmin".to_owned(),
        device_model: Some("Forerunner 965 \u{1F3C3}\u{200D}\u{2642}\u{FE0F}".to_owned()),
        software_version: None,
        source: None,
        device_type: DeviceType::Watch,
        original_source_name: Some("\u{5065}\u{5EB7}\u{30C7}\u{30FC}\u{30BF}".to_owned()),
    };

    assert!(ds.device_model.is_some());
    assert!(ds.original_source_name.is_some());

    // Roundtrip through JSON preserves Unicode
    let json = serde_json::to_string(&ds).expect("Unicode data source should serialize");
    let roundtripped: DataSource =
        serde_json::from_str(&json).expect("Unicode JSON should deserialize");
    assert_eq!(roundtripped.device_model, ds.device_model);
    assert_eq!(roundtripped.original_source_name, ds.original_source_name);
}

#[test]
fn test_max_f64_weight() {
    let metrics = StoredHealthMetrics {
        id: "hlth-max".to_owned(),
        user_id: "user-001".to_owned(),
        data_source_id: "ds-001".to_owned(),
        date: NaiveDate::from_ymd_opt(2026, 3, 24).expect("valid date"),
        weight_kg: Some(f64::MAX),
        body_fat_pct: None,
        muscle_mass_kg: None,
        bmi: None,
        bone_mass_kg: None,
        water_pct: None,
        systolic_bp: None,
        diastolic_bp: None,
        blood_glucose: None,
        source_name: "Test".to_owned(),
        recorded_at: Utc::now(),
    };

    assert_eq!(metrics.weight_kg, Some(f64::MAX));
}

#[test]
fn test_date_at_epoch() {
    let epoch = NaiveDate::from_ymd_opt(1970, 1, 1).expect("epoch date");
    let metrics = StoredRecoveryMetrics {
        id: "rec-epoch".to_owned(),
        user_id: "user-001".to_owned(),
        data_source_id: "ds-001".to_owned(),
        date: epoch,
        recovery_score: Some(50),
        readiness_score: None,
        hrv_ms: None,
        hrv_rmssd: None,
        resting_heart_rate: None,
        stress_score: None,
        body_battery: None,
        spo2: None,
        respiratory_rate: None,
        skin_temp_deviation: None,
        source_name: "Test".to_owned(),
        recorded_at: Utc.with_ymd_and_hms(1970, 1, 1, 0, 0, 0).unwrap(),
    };

    assert_eq!(metrics.date, epoch);
}

#[test]
fn test_date_far_future() {
    let future = NaiveDate::from_ymd_opt(2099, 12, 31).expect("far future date");
    let metrics = StoredRecoveryMetrics {
        id: "rec-future".to_owned(),
        user_id: "user-001".to_owned(),
        data_source_id: "ds-001".to_owned(),
        date: future,
        recovery_score: Some(99),
        readiness_score: None,
        hrv_ms: None,
        hrv_rmssd: None,
        resting_heart_rate: None,
        stress_score: None,
        body_battery: None,
        spo2: None,
        respiratory_rate: None,
        skin_temp_deviation: None,
        source_name: "Test".to_owned(),
        recorded_at: Utc::now(),
    };

    assert_eq!(metrics.date, future);
}

#[test]
fn test_sleep_session_spanning_midnight() {
    // Session starts at 23:00 on March 24 and ends at 07:00 on March 25
    let start = Utc.with_ymd_and_hms(2026, 3, 24, 23, 0, 0).unwrap();
    let end = Utc.with_ymd_and_hms(2026, 3, 25, 7, 0, 0).unwrap();

    let session = StoredSleepSession {
        id: "sleep-midnight".to_owned(),
        user_id: "user-001".to_owned(),
        data_source_id: "ds-001".to_owned(),
        is_nap: false,
        start_datetime: start,
        end_datetime: end,
        total_sleep_seconds: Some(28800),
        deep_sleep_seconds: None,
        light_sleep_seconds: None,
        rem_sleep_seconds: None,
        awake_seconds: None,
        sleep_efficiency: None,
        avg_heart_rate: None,
        min_heart_rate: None,
        avg_hrv: None,
        sleep_score: None,
        stages: vec![],
        source_name: "Test".to_owned(),
    };

    // Verify end is on a different day than start
    assert_ne!(
        session.start_datetime.date_naive(),
        session.end_datetime.date_naive(),
        "midnight-spanning session should cross date boundary"
    );
    assert!(session.end_datetime > session.start_datetime);
}

#[test]
fn test_sleep_session_zero_duration() {
    let now = Utc::now();
    let session = StoredSleepSession {
        id: "sleep-zero".to_owned(),
        user_id: "user-001".to_owned(),
        data_source_id: "ds-001".to_owned(),
        is_nap: true,
        start_datetime: now,
        end_datetime: now,
        total_sleep_seconds: Some(0),
        deep_sleep_seconds: None,
        light_sleep_seconds: None,
        rem_sleep_seconds: None,
        awake_seconds: None,
        sleep_efficiency: None,
        avg_heart_rate: None,
        min_heart_rate: None,
        avg_hrv: None,
        sleep_score: None,
        stages: vec![],
        source_name: "Test".to_owned(),
    };

    assert_eq!(session.total_sleep_seconds, Some(0));
    let duration = session
        .end_datetime
        .signed_duration_since(session.start_datetime);
    assert_eq!(duration, Duration::zero());
}

#[test]
fn test_recovery_all_scores_zero() {
    let metrics = StoredRecoveryMetrics {
        id: "rec-zero".to_owned(),
        user_id: "user-001".to_owned(),
        data_source_id: "ds-001".to_owned(),
        date: NaiveDate::from_ymd_opt(2026, 3, 24).expect("valid date"),
        recovery_score: Some(0),
        readiness_score: Some(0),
        hrv_ms: Some(0.0),
        hrv_rmssd: Some(0.0),
        resting_heart_rate: Some(0),
        stress_score: Some(0),
        body_battery: Some(0),
        spo2: Some(0.0),
        respiratory_rate: Some(0.0),
        skin_temp_deviation: Some(0.0),
        source_name: "Test".to_owned(),
        recorded_at: Utc::now(),
    };

    assert_eq!(metrics.recovery_score, Some(0));
    assert_eq!(metrics.readiness_score, Some(0));
    assert_eq!(metrics.stress_score, Some(0));
    assert_eq!(metrics.body_battery, Some(0));
}

#[test]
fn test_recovery_all_scores_hundred() {
    let metrics = StoredRecoveryMetrics {
        id: "rec-max".to_owned(),
        user_id: "user-001".to_owned(),
        data_source_id: "ds-001".to_owned(),
        date: NaiveDate::from_ymd_opt(2026, 3, 24).expect("valid date"),
        recovery_score: Some(100),
        readiness_score: Some(100),
        hrv_ms: Some(200.0),
        hrv_rmssd: Some(180.0),
        resting_heart_rate: Some(100),
        stress_score: Some(100),
        body_battery: Some(100),
        spo2: Some(100.0),
        respiratory_rate: Some(40.0),
        skin_temp_deviation: Some(2.0),
        source_name: "Test".to_owned(),
        recorded_at: Utc::now(),
    };

    assert_eq!(metrics.recovery_score, Some(100));
    assert_eq!(metrics.readiness_score, Some(100));
    assert_eq!(metrics.stress_score, Some(100));
    assert_eq!(metrics.body_battery, Some(100));
}

#[test]
fn test_health_snapshot_very_light_weight() {
    let metrics = StoredHealthMetrics {
        id: "hlth-light".to_owned(),
        user_id: "user-001".to_owned(),
        data_source_id: "ds-001".to_owned(),
        date: NaiveDate::from_ymd_opt(2026, 3, 24).expect("valid date"),
        weight_kg: Some(2.5),
        body_fat_pct: Some(5.0),
        muscle_mass_kg: Some(1.0),
        bmi: Some(12.0),
        bone_mass_kg: Some(0.5),
        water_pct: Some(70.0),
        systolic_bp: None,
        diastolic_bp: None,
        blood_glucose: None,
        source_name: "Test".to_owned(),
        recorded_at: Utc::now(),
    };

    assert_eq!(metrics.weight_kg, Some(2.5));
}

#[test]
fn test_health_snapshot_very_heavy_weight() {
    let metrics = StoredHealthMetrics {
        id: "hlth-heavy".to_owned(),
        user_id: "user-001".to_owned(),
        data_source_id: "ds-001".to_owned(),
        date: NaiveDate::from_ymd_opt(2026, 3, 24).expect("valid date"),
        weight_kg: Some(500.0),
        body_fat_pct: Some(55.0),
        muscle_mass_kg: Some(100.0),
        bmi: Some(70.0),
        bone_mass_kg: Some(10.0),
        water_pct: Some(40.0),
        systolic_bp: Some(200),
        diastolic_bp: Some(120),
        blood_glucose: Some(300.0),
        source_name: "Test".to_owned(),
        recorded_at: Utc::now(),
    };

    assert_eq!(metrics.weight_kg, Some(500.0));
}
