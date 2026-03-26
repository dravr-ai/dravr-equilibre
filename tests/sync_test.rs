// ABOUTME: Tests for sync lifecycle models including status transitions and result tracking
// ABOUTME: Validates SyncStatus states, SyncResult construction, and JSON roundtrips
//
// SPDX-License-Identifier: Apache-2.0
// Copyright (c) 2026 dravr.ai

use chrono::{Duration, Utc};
use dravr_equilibre::sync::{SyncResult, SyncStatus};

#[test]
fn test_sync_status_pending() {
    let status = SyncStatus::Pending;
    assert_eq!(status, SyncStatus::Pending);
    assert_eq!(status.to_string(), "Pending");
}

#[test]
fn test_sync_status_in_progress() {
    let status = SyncStatus::InProgress;
    assert_eq!(status, SyncStatus::InProgress);
    assert_eq!(status.to_string(), "InProgress");
}

#[test]
fn test_sync_status_completed() {
    let status = SyncStatus::Completed;
    assert_eq!(status, SyncStatus::Completed);
    assert_eq!(status.to_string(), "Completed");
}

#[test]
fn test_sync_status_failed() {
    let status = SyncStatus::Failed;
    assert_eq!(status, SyncStatus::Failed);
    assert_eq!(status.to_string(), "Failed");
}

#[test]
fn test_sync_status_cancelled() {
    let status = SyncStatus::Cancelled;
    assert_eq!(status, SyncStatus::Cancelled);
    assert_eq!(status.to_string(), "Cancelled");
}

#[test]
fn test_sync_status_serialization_roundtrip_all_states() {
    let statuses = [
        SyncStatus::Pending,
        SyncStatus::InProgress,
        SyncStatus::Completed,
        SyncStatus::Failed,
        SyncStatus::Cancelled,
    ];

    for status in statuses {
        let json = serde_json::to_string(&status).expect("SyncStatus should serialize");
        let roundtripped: SyncStatus =
            serde_json::from_str(&json).expect("SyncStatus should deserialize");
        assert_eq!(roundtripped, status, "Roundtrip failed for {status:?}");
    }
}

#[test]
fn test_sync_result_successful() {
    let started = Utc::now();
    let completed = started + Duration::seconds(45);
    let result = SyncResult {
        status: SyncStatus::Completed,
        provider: "garmin".to_owned(),
        user_id: "user-001".to_owned(),
        records_created: 25,
        records_updated: 5,
        records_skipped: 100,
        records_errored: 0,
        error_message: None,
        started_at: started,
        completed_at: Some(completed),
    };

    assert_eq!(result.status, SyncStatus::Completed);
    assert_eq!(result.records_created, 25);
    assert_eq!(result.records_updated, 5);
    assert_eq!(result.records_skipped, 100);
    assert_eq!(result.records_errored, 0);
    assert!(result.error_message.is_none());
    assert!(result.completed_at.is_some());
}

#[test]
fn test_sync_result_failed_with_error() {
    let started = Utc::now();
    let result = SyncResult {
        status: SyncStatus::Failed,
        provider: "strava".to_owned(),
        user_id: "user-002".to_owned(),
        records_created: 0,
        records_updated: 0,
        records_skipped: 0,
        records_errored: 10,
        error_message: Some("OAuth token expired and refresh failed".to_owned()),
        started_at: started,
        completed_at: Some(started + Duration::seconds(3)),
    };

    assert_eq!(result.status, SyncStatus::Failed);
    assert_eq!(result.records_errored, 10);
    assert_eq!(
        result.error_message.as_deref(),
        Some("OAuth token expired and refresh failed")
    );
}

#[test]
fn test_sync_result_zero_records() {
    let started = Utc::now();
    let result = SyncResult {
        status: SyncStatus::Completed,
        provider: "apple".to_owned(),
        user_id: "user-003".to_owned(),
        records_created: 0,
        records_updated: 0,
        records_skipped: 0,
        records_errored: 0,
        error_message: None,
        started_at: started,
        completed_at: Some(started + Duration::seconds(1)),
    };

    assert_eq!(result.status, SyncStatus::Completed);
    let total = result.records_created
        + result.records_updated
        + result.records_skipped
        + result.records_errored;
    assert_eq!(total, 0, "all record counts should be zero");
}

#[test]
fn test_sync_result_large_record_count() {
    let started = Utc::now();
    let result = SyncResult {
        status: SyncStatus::Completed,
        provider: "garmin".to_owned(),
        user_id: "user-004".to_owned(),
        records_created: 500_000,
        records_updated: 150_000,
        records_skipped: 1_000_000,
        records_errored: 50,
        error_message: None,
        started_at: started,
        completed_at: Some(started + Duration::minutes(30)),
    };

    assert_eq!(result.records_created, 500_000);
    assert_eq!(result.records_skipped, 1_000_000);
}

#[test]
fn test_sync_result_serialization_roundtrip() {
    let started = Utc::now();
    let result = SyncResult {
        status: SyncStatus::Completed,
        provider: "polar".to_owned(),
        user_id: "user-005".to_owned(),
        records_created: 10,
        records_updated: 2,
        records_skipped: 5,
        records_errored: 0,
        error_message: None,
        started_at: started,
        completed_at: Some(started + Duration::seconds(15)),
    };

    let json = serde_json::to_string(&result).expect("SyncResult should serialize to JSON");
    let roundtripped: SyncResult =
        serde_json::from_str(&json).expect("JSON should deserialize back to SyncResult");

    assert_eq!(roundtripped.status, result.status);
    assert_eq!(roundtripped.provider, result.provider);
    assert_eq!(roundtripped.records_created, result.records_created);
    assert_eq!(roundtripped.records_updated, result.records_updated);
    assert_eq!(roundtripped.records_skipped, result.records_skipped);
    assert_eq!(roundtripped.records_errored, result.records_errored);
    assert!(roundtripped.error_message.is_none());
}

#[test]
fn test_sync_result_error_message_none_in_json() {
    let started = Utc::now();
    let result = SyncResult {
        status: SyncStatus::Completed,
        provider: "garmin".to_owned(),
        user_id: "user-006".to_owned(),
        records_created: 1,
        records_updated: 0,
        records_skipped: 0,
        records_errored: 0,
        error_message: None,
        started_at: started,
        completed_at: None,
    };

    let json = serde_json::to_string(&result).expect("SyncResult should serialize");

    // error_message is still present as null (no skip_serializing_if on this struct)
    // Verify the roundtrip preserves None
    let roundtripped: SyncResult =
        serde_json::from_str(&json).expect("SyncResult should deserialize");
    assert!(roundtripped.error_message.is_none());
}
