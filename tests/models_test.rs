// ABOUTME: Tests for health domain models including events, workouts, sleep, recovery, and health
// ABOUTME: Validates serialization, construction, and field correctness of all domain types
//
// SPDX-License-Identifier: Apache-2.0
// Copyright (c) 2026 dravr.ai

#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::str_to_string
)]

use chrono::{NaiveDate, Utc};
use dravr_equilibre::data_source::{DataSource, DeviceType};
use dravr_equilibre::error::EquilibreError;
use dravr_equilibre::event::{EventCategory, EventRecord};
use dravr_equilibre::health::StoredHealthMetrics;
use dravr_equilibre::recovery::StoredRecoveryMetrics;
use dravr_equilibre::sleep::{SleepStage, SleepStageType, StoredSleepSession};
use dravr_equilibre::sync::{SyncResult, SyncStatus};
use dravr_equilibre::workout::WorkoutDetails;

#[test]
fn test_device_type_display() {
    assert_eq!(DeviceType::Watch.to_string(), "Watch");
    assert_eq!(DeviceType::Band.to_string(), "Band");
    assert_eq!(DeviceType::Phone.to_string(), "Phone");
    assert_eq!(DeviceType::Ring.to_string(), "Ring");
    assert_eq!(DeviceType::Scale.to_string(), "Scale");
    assert_eq!(DeviceType::Unknown.to_string(), "Unknown");
}

#[test]
fn test_device_type_serialization_roundtrip() {
    let device_type = DeviceType::Watch;
    let json = serde_json::to_string(&device_type).unwrap();
    let deserialized: DeviceType = serde_json::from_str(&json).unwrap();
    assert_eq!(deserialized, device_type);
}

#[test]
fn test_data_source_construction() {
    let ds = DataSource {
        id: "ds-001".to_owned(),
        user_id: "user-abc".to_owned(),
        provider: "garmin".to_owned(),
        device_model: Some("Forerunner 965".to_owned()),
        software_version: Some("19.20".to_owned()),
        source: None,
        device_type: DeviceType::Watch,
        original_source_name: Some("Garmin Connect".to_owned()),
    };

    assert_eq!(ds.id, "ds-001");
    assert_eq!(ds.provider, "garmin");
    assert_eq!(ds.device_type, DeviceType::Watch);
    assert_eq!(ds.device_model.as_deref(), Some("Forerunner 965"));
}

#[test]
fn test_data_source_serialization_roundtrip() {
    let ds = DataSource {
        id: "ds-002".to_owned(),
        user_id: "user-xyz".to_owned(),
        provider: "apple".to_owned(),
        device_model: Some("Apple Watch Series 9".to_owned()),
        software_version: Some("10.3".to_owned()),
        source: Some("com.apple.health".to_owned()),
        device_type: DeviceType::Watch,
        original_source_name: None,
    };

    let json = serde_json::to_string(&ds).unwrap();
    let deserialized: DataSource = serde_json::from_str(&json).unwrap();
    assert_eq!(deserialized.id, ds.id);
    assert_eq!(deserialized.provider, ds.provider);
    assert_eq!(deserialized.device_type, ds.device_type);
}

#[test]
fn test_event_category_display() {
    assert_eq!(EventCategory::Workout.to_string(), "Workout");
    assert_eq!(EventCategory::Sleep.to_string(), "Sleep");
}

#[test]
fn test_event_record_construction() {
    let now = Utc::now();
    let event = EventRecord {
        id: "evt-001".to_owned(),
        external_id: Some("strava-12345".to_owned()),
        data_source_id: "ds-001".to_owned(),
        category: EventCategory::Workout,
        event_type: "Run".to_owned(),
        source_name: "Strava".to_owned(),
        duration_seconds: Some(3600),
        start_datetime: now,
        end_datetime: now + chrono::Duration::hours(1),
        zone_offset: Some("+05:00".to_owned()),
    };

    assert_eq!(event.id, "evt-001");
    assert_eq!(event.category, EventCategory::Workout);
    assert_eq!(event.duration_seconds, Some(3600));
}

#[test]
fn test_workout_details_construction() {
    let now = Utc::now();
    let workout = WorkoutDetails {
        event: EventRecord {
            id: "wkt-001".to_owned(),
            external_id: None,
            data_source_id: "ds-001".to_owned(),
            category: EventCategory::Workout,
            event_type: "Cycling".to_owned(),
            source_name: "Garmin".to_owned(),
            duration_seconds: Some(5400),
            start_datetime: now,
            end_datetime: now + chrono::Duration::seconds(5400),
            zone_offset: None,
        },
        heart_rate_min: Some(90),
        heart_rate_max: Some(185),
        heart_rate_avg: Some(155.5),
        energy_burned: Some(850.0),
        distance: Some(40_000.0),
        steps_count: None,
        max_speed: Some(12.5),
        average_speed: Some(7.4),
        max_watts: Some(350),
        average_watts: Some(220),
        moving_time_seconds: Some(5200),
        total_elevation_gain: Some(650.0),
        elev_high: Some(1200.0),
        elev_low: Some(550.0),
    };

    assert_eq!(workout.heart_rate_max, Some(185));
    assert_eq!(workout.distance, Some(40_000.0));
    assert_eq!(workout.average_watts, Some(220));
}

#[test]
fn test_sleep_stage_type_display() {
    assert_eq!(SleepStageType::Awake.to_string(), "Awake");
    assert_eq!(SleepStageType::Light.to_string(), "Light");
    assert_eq!(SleepStageType::Deep.to_string(), "Deep");
    assert_eq!(SleepStageType::Rem.to_string(), "REM");
    assert_eq!(SleepStageType::Unknown.to_string(), "Unknown");
}

#[test]
fn test_stored_sleep_session_with_stages() {
    let now = Utc::now();
    let session = StoredSleepSession {
        id: "sleep-001".to_owned(),
        user_id: "user-abc".to_owned(),
        data_source_id: "ds-001".to_owned(),
        is_nap: false,
        start_datetime: now,
        end_datetime: now + chrono::Duration::hours(8),
        total_sleep_seconds: Some(27000),
        deep_sleep_seconds: Some(5400),
        light_sleep_seconds: Some(14400),
        rem_sleep_seconds: Some(7200),
        awake_seconds: Some(1800),
        sleep_efficiency: Some(93.75),
        avg_heart_rate: Some(58.0),
        min_heart_rate: Some(48),
        avg_hrv: Some(45.0),
        sleep_score: Some(85),
        stages: vec![
            SleepStage {
                stage_type: SleepStageType::Light,
                start: now,
                end: now + chrono::Duration::minutes(30),
                duration_seconds: 1800,
            },
            SleepStage {
                stage_type: SleepStageType::Deep,
                start: now + chrono::Duration::minutes(30),
                end: now + chrono::Duration::minutes(90),
                duration_seconds: 3600,
            },
        ],
        source_name: "Garmin".to_owned(),
    };

    assert!(!session.is_nap);
    assert_eq!(session.stages.len(), 2);
    assert_eq!(session.stages[0].stage_type, SleepStageType::Light);
    assert_eq!(session.stages[1].stage_type, SleepStageType::Deep);
    assert_eq!(session.sleep_score, Some(85));
}

#[test]
fn test_stored_sleep_session_nap() {
    let now = Utc::now();
    let nap = StoredSleepSession {
        id: "sleep-002".to_owned(),
        user_id: "user-abc".to_owned(),
        data_source_id: "ds-001".to_owned(),
        is_nap: true,
        start_datetime: now,
        end_datetime: now + chrono::Duration::minutes(25),
        total_sleep_seconds: Some(1500),
        deep_sleep_seconds: None,
        light_sleep_seconds: Some(1500),
        rem_sleep_seconds: None,
        awake_seconds: None,
        sleep_efficiency: None,
        avg_heart_rate: None,
        min_heart_rate: None,
        avg_hrv: None,
        sleep_score: None,
        stages: vec![],
        source_name: "Apple".to_owned(),
    };

    assert!(nap.is_nap);
    assert!(nap.stages.is_empty());
}

#[test]
fn test_sleep_stages_json_roundtrip() {
    let now = Utc::now();
    let stages = vec![
        SleepStage {
            stage_type: SleepStageType::Light,
            start: now,
            end: now + chrono::Duration::minutes(30),
            duration_seconds: 1800,
        },
        SleepStage {
            stage_type: SleepStageType::Rem,
            start: now + chrono::Duration::minutes(30),
            end: now + chrono::Duration::minutes(60),
            duration_seconds: 1800,
        },
    ];

    let json = serde_json::to_string(&stages).unwrap();
    let deserialized: Vec<SleepStage> = serde_json::from_str(&json).unwrap();
    assert_eq!(deserialized.len(), 2);
    assert_eq!(deserialized[0].stage_type, SleepStageType::Light);
    assert_eq!(deserialized[1].stage_type, SleepStageType::Rem);
}

#[test]
fn test_stored_recovery_metrics() {
    let metrics = StoredRecoveryMetrics {
        id: "rec-001".to_owned(),
        user_id: "user-abc".to_owned(),
        data_source_id: "ds-001".to_owned(),
        date: NaiveDate::from_ymd_opt(2026, 3, 24).unwrap(),
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
    };

    assert_eq!(metrics.recovery_score, Some(78));
    assert_eq!(metrics.resting_heart_rate, Some(52));
    assert_eq!(metrics.date, NaiveDate::from_ymd_opt(2026, 3, 24).unwrap());
}

#[test]
fn test_stored_health_metrics() {
    let metrics = StoredHealthMetrics {
        id: "hlth-001".to_owned(),
        user_id: "user-abc".to_owned(),
        data_source_id: "ds-001".to_owned(),
        date: NaiveDate::from_ymd_opt(2026, 3, 24).unwrap(),
        weight_kg: Some(75.5),
        body_fat_pct: Some(18.2),
        muscle_mass_kg: Some(35.0),
        bmi: Some(23.8),
        bone_mass_kg: Some(3.2),
        water_pct: Some(55.0),
        systolic_bp: Some(120),
        diastolic_bp: Some(80),
        blood_glucose: Some(95.0),
        source_name: "Withings".to_owned(),
        recorded_at: Utc::now(),
    };

    assert_eq!(metrics.weight_kg, Some(75.5));
    assert_eq!(metrics.body_fat_pct, Some(18.2));
    assert_eq!(metrics.systolic_bp, Some(120));
}

#[test]
fn test_sync_status_display() {
    assert_eq!(SyncStatus::Pending.to_string(), "Pending");
    assert_eq!(SyncStatus::InProgress.to_string(), "InProgress");
    assert_eq!(SyncStatus::Completed.to_string(), "Completed");
    assert_eq!(SyncStatus::Failed.to_string(), "Failed");
    assert_eq!(SyncStatus::Cancelled.to_string(), "Cancelled");
}

#[test]
fn test_sync_result_construction() {
    let now = Utc::now();
    let result = SyncResult {
        status: SyncStatus::Completed,
        provider: "garmin".to_owned(),
        user_id: "user-abc".to_owned(),
        records_created: 15,
        records_updated: 3,
        records_skipped: 42,
        records_errored: 0,
        error_message: None,
        started_at: now,
        completed_at: Some(now + chrono::Duration::seconds(30)),
    };

    assert_eq!(result.status, SyncStatus::Completed);
    assert_eq!(result.records_created, 15);
    assert!(result.error_message.is_none());
}

#[test]
fn test_sync_result_failed() {
    let now = Utc::now();
    let result = SyncResult {
        status: SyncStatus::Failed,
        provider: "strava".to_owned(),
        user_id: "user-xyz".to_owned(),
        records_created: 0,
        records_updated: 0,
        records_skipped: 0,
        records_errored: 5,
        error_message: Some("Rate limit exceeded".to_owned()),
        started_at: now,
        completed_at: Some(now + chrono::Duration::seconds(2)),
    };

    assert_eq!(result.status, SyncStatus::Failed);
    assert_eq!(result.error_message.as_deref(), Some("Rate limit exceeded"));
}

#[test]
fn test_error_display_messages() {
    let err = EquilibreError::missing_field("user_id");
    assert_eq!(err.to_string(), "missing required field: 'user_id'");

    let err = EquilibreError::out_of_range("heart_rate", 300.0, 0.0, 250.0);
    assert!(err.to_string().contains("300"));
    assert!(err.to_string().contains("heart_rate"));

    let err = EquilibreError::stages_serialization("invalid JSON");
    assert!(err.to_string().contains("invalid JSON"));

    let err = EquilibreError::InvalidTimeRange;
    assert!(err.to_string().contains("start must be before end"));

    let err = EquilibreError::provider_not_supported("strava", "continuous_data");
    assert!(err.to_string().contains("strava"));
    assert!(err.to_string().contains("continuous_data"));

    let err = EquilibreError::duplicate_data_source("ds-001");
    assert!(err.to_string().contains("ds-001"));
}

#[test]
fn test_continuous_metric_batch_serialization() {
    use dravr_equilibre::provider::ContinuousMetricBatch;

    let now = Utc::now();
    let batch = ContinuousMetricBatch {
        series_type_id: 42,
        points: vec![(now, 72.0), (now + chrono::Duration::minutes(1), 73.5)],
    };

    let json = serde_json::to_string(&batch).unwrap();
    let deserialized: ContinuousMetricBatch = serde_json::from_str(&json).unwrap();
    assert_eq!(deserialized.series_type_id, 42);
    assert_eq!(deserialized.points.len(), 2);
}
