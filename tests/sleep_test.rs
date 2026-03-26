// ABOUTME: Tests for sleep session models including stages, nap detection, and serialization
// ABOUTME: Validates StoredSleepSession construction, SleepStage types, and JSON roundtrips
//
// SPDX-License-Identifier: Apache-2.0
// Copyright (c) 2026 dravr.ai

use chrono::{Duration, TimeZone, Utc};
use dravr_equilibre::event::{EventCategory, EventRecord};
use dravr_equilibre::sleep::{SleepDetails, SleepStage, SleepStageType, StoredSleepSession};

/// Helper to create a fully populated sleep session for reuse across tests.
fn full_sleep_session() -> StoredSleepSession {
    let start = Utc.with_ymd_and_hms(2026, 3, 24, 22, 0, 0).unwrap();
    let end = start + Duration::hours(8);
    StoredSleepSession {
        id: "sleep-full".to_owned(),
        user_id: "user-001".to_owned(),
        data_source_id: "ds-garmin".to_owned(),
        is_nap: false,
        start_datetime: start,
        end_datetime: end,
        total_sleep_seconds: Some(27000),
        deep_sleep_seconds: Some(5400),
        light_sleep_seconds: Some(14400),
        rem_sleep_seconds: Some(7200),
        awake_seconds: Some(1800),
        sleep_efficiency: Some(93.75),
        avg_heart_rate: Some(56.0),
        min_heart_rate: Some(45),
        avg_hrv: Some(48.0),
        sleep_score: Some(88),
        stages: vec![
            SleepStage {
                stage_type: SleepStageType::Light,
                start,
                end: start + Duration::minutes(30),
                duration_seconds: 1800,
            },
            SleepStage {
                stage_type: SleepStageType::Deep,
                start: start + Duration::minutes(30),
                end: start + Duration::minutes(90),
                duration_seconds: 3600,
            },
            SleepStage {
                stage_type: SleepStageType::Rem,
                start: start + Duration::minutes(90),
                end: start + Duration::minutes(120),
                duration_seconds: 1800,
            },
        ],
        source_name: "Garmin".to_owned(),
    }
}

#[test]
fn test_stored_sleep_session_all_fields_populated() {
    let session = full_sleep_session();

    assert_eq!(session.id, "sleep-full");
    assert_eq!(session.user_id, "user-001");
    assert_eq!(session.data_source_id, "ds-garmin");
    assert!(!session.is_nap);
    assert_eq!(session.total_sleep_seconds, Some(27000));
    assert_eq!(session.deep_sleep_seconds, Some(5400));
    assert_eq!(session.light_sleep_seconds, Some(14400));
    assert_eq!(session.rem_sleep_seconds, Some(7200));
    assert_eq!(session.awake_seconds, Some(1800));
    assert_eq!(session.sleep_efficiency, Some(93.75));
    assert_eq!(session.avg_heart_rate, Some(56.0));
    assert_eq!(session.min_heart_rate, Some(45));
    assert_eq!(session.avg_hrv, Some(48.0));
    assert_eq!(session.sleep_score, Some(88));
    assert_eq!(session.stages.len(), 3);
    assert_eq!(session.source_name, "Garmin");
}

#[test]
fn test_stored_sleep_session_minimal_fields() {
    let start = Utc::now();
    let session = StoredSleepSession {
        id: "sleep-min".to_owned(),
        user_id: "user-002".to_owned(),
        data_source_id: "ds-apple".to_owned(),
        is_nap: false,
        start_datetime: start,
        end_datetime: start + Duration::hours(7),
        total_sleep_seconds: None,
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
        source_name: "Apple".to_owned(),
    };

    assert!(session.total_sleep_seconds.is_none());
    assert!(session.deep_sleep_seconds.is_none());
    assert!(session.light_sleep_seconds.is_none());
    assert!(session.rem_sleep_seconds.is_none());
    assert!(session.awake_seconds.is_none());
    assert!(session.sleep_efficiency.is_none());
    assert!(session.avg_heart_rate.is_none());
    assert!(session.min_heart_rate.is_none());
    assert!(session.avg_hrv.is_none());
    assert!(session.sleep_score.is_none());
    assert!(session.stages.is_empty());
}

#[test]
fn test_is_nap_true() {
    let start = Utc::now();
    let nap = StoredSleepSession {
        id: "nap-001".to_owned(),
        user_id: "user-001".to_owned(),
        data_source_id: "ds-001".to_owned(),
        is_nap: true,
        start_datetime: start,
        end_datetime: start + Duration::minutes(20),
        total_sleep_seconds: Some(1200),
        deep_sleep_seconds: None,
        light_sleep_seconds: Some(1200),
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
}

#[test]
fn test_is_nap_false() {
    let session = full_sleep_session();
    assert!(!session.is_nap);
}

#[test]
fn test_stages_empty_vec() {
    let start = Utc::now();
    let session = StoredSleepSession {
        id: "sleep-empty-stages".to_owned(),
        user_id: "user-001".to_owned(),
        data_source_id: "ds-001".to_owned(),
        is_nap: false,
        start_datetime: start,
        end_datetime: start + Duration::hours(7),
        total_sleep_seconds: None,
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
        source_name: "Unknown".to_owned(),
    };

    // Empty stages serializes to "[]"
    let json = serde_json::to_string(&session.stages).expect("empty stages vec should serialize");
    assert_eq!(json, "[]");
}

#[test]
fn test_stages_all_stage_types() {
    let start = Utc::now();
    let stages = [
        SleepStage {
            stage_type: SleepStageType::Awake,
            start,
            end: start + Duration::minutes(5),
            duration_seconds: 300,
        },
        SleepStage {
            stage_type: SleepStageType::Light,
            start: start + Duration::minutes(5),
            end: start + Duration::minutes(35),
            duration_seconds: 1800,
        },
        SleepStage {
            stage_type: SleepStageType::Deep,
            start: start + Duration::minutes(35),
            end: start + Duration::minutes(65),
            duration_seconds: 1800,
        },
        SleepStage {
            stage_type: SleepStageType::Rem,
            start: start + Duration::minutes(65),
            end: start + Duration::minutes(95),
            duration_seconds: 1800,
        },
        SleepStage {
            stage_type: SleepStageType::Unknown,
            start: start + Duration::minutes(95),
            end: start + Duration::minutes(100),
            duration_seconds: 300,
        },
    ];

    assert_eq!(stages[0].stage_type, SleepStageType::Awake);
    assert_eq!(stages[1].stage_type, SleepStageType::Light);
    assert_eq!(stages[2].stage_type, SleepStageType::Deep);
    assert_eq!(stages[3].stage_type, SleepStageType::Rem);
    assert_eq!(stages[4].stage_type, SleepStageType::Unknown);
}

#[test]
fn test_sleep_stage_all_five_types() {
    let stage_types = [
        SleepStageType::Awake,
        SleepStageType::Light,
        SleepStageType::Deep,
        SleepStageType::Rem,
        SleepStageType::Unknown,
    ];

    for stage_type in stage_types {
        let display = stage_type.to_string();
        assert!(
            !display.is_empty(),
            "Display should produce non-empty string for {stage_type:?}"
        );
    }
}

#[test]
fn test_sleep_stage_type_serialization() {
    let types = [
        (SleepStageType::Awake, "\"Awake\""),
        (SleepStageType::Light, "\"Light\""),
        (SleepStageType::Deep, "\"Deep\""),
        (SleepStageType::Rem, "\"Rem\""),
        (SleepStageType::Unknown, "\"Unknown\""),
    ];

    for (variant, expected_json) in types {
        let json = serde_json::to_string(&variant).expect("SleepStageType should serialize");
        assert_eq!(
            json, expected_json,
            "Serialization mismatch for {variant:?}"
        );
    }
}

#[test]
fn test_sleep_stage_type_deserialization() {
    let cases = [
        ("\"Awake\"", SleepStageType::Awake),
        ("\"Light\"", SleepStageType::Light),
        ("\"Deep\"", SleepStageType::Deep),
        ("\"Rem\"", SleepStageType::Rem),
        ("\"Unknown\"", SleepStageType::Unknown),
    ];

    for (json_str, expected) in cases {
        let deserialized: SleepStageType =
            serde_json::from_str(json_str).expect("SleepStageType should deserialize");
        assert_eq!(
            deserialized, expected,
            "Deserialization mismatch for {json_str}"
        );
    }
}

#[test]
fn test_sleep_efficiency_boundary_zero() {
    let start = Utc::now();
    let session = StoredSleepSession {
        id: "sleep-eff-0".to_owned(),
        user_id: "user-001".to_owned(),
        data_source_id: "ds-001".to_owned(),
        is_nap: false,
        start_datetime: start,
        end_datetime: start + Duration::hours(8),
        total_sleep_seconds: Some(0),
        deep_sleep_seconds: None,
        light_sleep_seconds: None,
        rem_sleep_seconds: None,
        awake_seconds: Some(28800),
        sleep_efficiency: Some(0.0),
        avg_heart_rate: None,
        min_heart_rate: None,
        avg_hrv: None,
        sleep_score: None,
        stages: vec![],
        source_name: "Test".to_owned(),
    };

    assert_eq!(session.sleep_efficiency, Some(0.0));
}

#[test]
fn test_sleep_efficiency_boundary_hundred() {
    let start = Utc::now();
    let session = StoredSleepSession {
        id: "sleep-eff-100".to_owned(),
        user_id: "user-001".to_owned(),
        data_source_id: "ds-001".to_owned(),
        is_nap: false,
        start_datetime: start,
        end_datetime: start + Duration::hours(8),
        total_sleep_seconds: Some(28800),
        deep_sleep_seconds: None,
        light_sleep_seconds: None,
        rem_sleep_seconds: None,
        awake_seconds: Some(0),
        sleep_efficiency: Some(100.0),
        avg_heart_rate: None,
        min_heart_rate: None,
        avg_hrv: None,
        sleep_score: None,
        stages: vec![],
        source_name: "Test".to_owned(),
    };

    assert_eq!(session.sleep_efficiency, Some(100.0));
}

#[test]
fn test_sleep_efficiency_boundary_midpoint() {
    let start = Utc::now();
    let session = StoredSleepSession {
        id: "sleep-eff-50".to_owned(),
        user_id: "user-001".to_owned(),
        data_source_id: "ds-001".to_owned(),
        is_nap: false,
        start_datetime: start,
        end_datetime: start + Duration::hours(8),
        total_sleep_seconds: Some(14400),
        deep_sleep_seconds: None,
        light_sleep_seconds: None,
        rem_sleep_seconds: None,
        awake_seconds: Some(14400),
        sleep_efficiency: Some(50.0),
        avg_heart_rate: None,
        min_heart_rate: None,
        avg_hrv: None,
        sleep_score: None,
        stages: vec![],
        source_name: "Test".to_owned(),
    };

    assert_eq!(session.sleep_efficiency, Some(50.0));
}

#[test]
fn test_sleep_score_boundary_zero() {
    let start = Utc::now();
    let session = StoredSleepSession {
        id: "sleep-score-0".to_owned(),
        user_id: "user-001".to_owned(),
        data_source_id: "ds-001".to_owned(),
        is_nap: false,
        start_datetime: start,
        end_datetime: start + Duration::hours(8),
        total_sleep_seconds: None,
        deep_sleep_seconds: None,
        light_sleep_seconds: None,
        rem_sleep_seconds: None,
        awake_seconds: None,
        sleep_efficiency: None,
        avg_heart_rate: None,
        min_heart_rate: None,
        avg_hrv: None,
        sleep_score: Some(0),
        stages: vec![],
        source_name: "Test".to_owned(),
    };

    assert_eq!(session.sleep_score, Some(0));
}

#[test]
fn test_sleep_score_boundary_hundred() {
    let start = Utc::now();
    let session = StoredSleepSession {
        id: "sleep-score-100".to_owned(),
        user_id: "user-001".to_owned(),
        data_source_id: "ds-001".to_owned(),
        is_nap: false,
        start_datetime: start,
        end_datetime: start + Duration::hours(8),
        total_sleep_seconds: None,
        deep_sleep_seconds: None,
        light_sleep_seconds: None,
        rem_sleep_seconds: None,
        awake_seconds: None,
        sleep_efficiency: None,
        avg_heart_rate: None,
        min_heart_rate: None,
        avg_hrv: None,
        sleep_score: Some(100),
        stages: vec![],
        source_name: "Test".to_owned(),
    };

    assert_eq!(session.sleep_score, Some(100));
}

#[test]
fn test_sleep_score_none() {
    let start = Utc::now();
    let session = StoredSleepSession {
        id: "sleep-score-none".to_owned(),
        user_id: "user-001".to_owned(),
        data_source_id: "ds-001".to_owned(),
        is_nap: false,
        start_datetime: start,
        end_datetime: start + Duration::hours(8),
        total_sleep_seconds: None,
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

    assert!(session.sleep_score.is_none());
}

#[test]
fn test_json_serialization_roundtrip_full_session() {
    let session = full_sleep_session();
    let json = serde_json::to_string(&session).expect("full session should serialize to JSON");
    let roundtripped: StoredSleepSession =
        serde_json::from_str(&json).expect("JSON should deserialize back to StoredSleepSession");

    assert_eq!(roundtripped.id, session.id);
    assert_eq!(roundtripped.user_id, session.user_id);
    assert_eq!(roundtripped.is_nap, session.is_nap);
    assert_eq!(
        roundtripped.total_sleep_seconds,
        session.total_sleep_seconds
    );
    assert_eq!(roundtripped.sleep_score, session.sleep_score);
    assert_eq!(roundtripped.stages.len(), session.stages.len());
    assert_eq!(roundtripped.stages[0].stage_type, SleepStageType::Light);
    assert_eq!(roundtripped.stages[1].stage_type, SleepStageType::Deep);
    assert_eq!(roundtripped.stages[2].stage_type, SleepStageType::Rem);
}

#[test]
fn test_large_stages_array_full_night() {
    let start = Utc.with_ymd_and_hms(2026, 3, 24, 22, 0, 0).unwrap();
    let stage_types = [
        SleepStageType::Light,
        SleepStageType::Deep,
        SleepStageType::Rem,
        SleepStageType::Awake,
    ];

    // 24 stages of 20 minutes each (8 hours total)
    let stages: Vec<SleepStage> = (0..24)
        .map(|i| {
            let stage_start = start + Duration::minutes(i64::from(i) * 20);
            let stage_end = stage_start + Duration::minutes(20);
            SleepStage {
                stage_type: stage_types[i as usize % stage_types.len()],
                start: stage_start,
                end: stage_end,
                duration_seconds: 1200,
            }
        })
        .collect();

    let session = StoredSleepSession {
        id: "sleep-24-stages".to_owned(),
        user_id: "user-001".to_owned(),
        data_source_id: "ds-001".to_owned(),
        is_nap: false,
        start_datetime: start,
        end_datetime: start + Duration::hours(8),
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
        stages,
        source_name: "Test".to_owned(),
    };

    assert_eq!(session.stages.len(), 24);

    // Verify roundtrip with large stages array
    let json = serde_json::to_string(&session).expect("session with 24 stages should serialize");
    let roundtripped: StoredSleepSession =
        serde_json::from_str(&json).expect("large stages array should deserialize");
    assert_eq!(roundtripped.stages.len(), 24);
}

#[test]
fn test_sleep_details_construction() {
    let start = Utc::now();
    let details = SleepDetails {
        event: EventRecord {
            id: "evt-sleep-001".to_owned(),
            external_id: None,
            data_source_id: "ds-001".to_owned(),
            category: EventCategory::Sleep,
            event_type: "NightSleep".to_owned(),
            source_name: "Garmin".to_owned(),
            duration_seconds: Some(28800),
            start_datetime: start,
            end_datetime: start + Duration::hours(8),
            zone_offset: Some("-05:00".to_owned()),
        },
        is_nap: false,
        total_sleep_seconds: Some(27000),
        sleep_efficiency: Some(93.75),
        sleep_score: Some(85),
        stages: vec![SleepStage {
            stage_type: SleepStageType::Light,
            start,
            end: start + Duration::minutes(30),
            duration_seconds: 1800,
        }],
    };

    assert_eq!(details.event.category, EventCategory::Sleep);
    assert!(!details.is_nap);
    assert_eq!(details.total_sleep_seconds, Some(27000));
    assert_eq!(details.stages.len(), 1);
}
