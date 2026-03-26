// ABOUTME: Tests for provider composition including strategy building and credential management
// ABOUTME: Validates ProviderStrategy patterns, Credentials, and ContinuousMetricBatch models
//
// SPDX-License-Identifier: Apache-2.0
// Copyright (c) 2026 dravr.ai

use chrono::{Duration, Utc};
use dravr_equilibre::provider::{ContinuousMetricBatch, Credentials, ProviderStrategy};

#[test]
fn test_provider_strategy_empty() {
    let strategy = ProviderStrategy::new("empty-provider");

    assert_eq!(strategy.name, "empty-provider");
    assert!(!strategy.supports_oauth());
    assert!(!strategy.supports_workouts());
    assert!(!strategy.supports_continuous_data());
}

#[test]
fn test_provider_strategy_name_stored() {
    let strategy = ProviderStrategy::new("strava");
    assert_eq!(strategy.name, "strava");
}

#[test]
fn test_provider_strategy_supports_oauth_false_by_default() {
    let strategy = ProviderStrategy::new("test");
    assert!(!strategy.supports_oauth());
}

#[test]
fn test_provider_strategy_supports_workouts_false_by_default() {
    let strategy = ProviderStrategy::new("test");
    assert!(!strategy.supports_workouts());
}

#[test]
fn test_provider_strategy_supports_continuous_data_false_by_default() {
    let strategy = ProviderStrategy::new("test");
    assert!(!strategy.supports_continuous_data());
}

#[test]
fn test_credentials_all_fields() {
    let expires = Utc::now() + Duration::hours(6);
    let creds = Credentials {
        access_token: "tok_abc123xyz".to_owned(),
        refresh_token: Some("ref_def456uvw".to_owned()),
        expires_at: Some(expires),
        scopes: vec![
            "read".to_owned(),
            "activity:read".to_owned(),
            "profile:read".to_owned(),
        ],
    };

    assert_eq!(creds.access_token, "tok_abc123xyz");
    assert_eq!(creds.refresh_token.as_deref(), Some("ref_def456uvw"));
    assert!(creds.expires_at.is_some());
    assert_eq!(creds.scopes.len(), 3);
    assert_eq!(creds.scopes[0], "read");
    assert_eq!(creds.scopes[1], "activity:read");
    assert_eq!(creds.scopes[2], "profile:read");
}

#[test]
fn test_credentials_minimal() {
    let creds = Credentials {
        access_token: "tok_minimal".to_owned(),
        refresh_token: None,
        expires_at: None,
        scopes: vec![],
    };

    assert_eq!(creds.access_token, "tok_minimal");
    assert!(creds.refresh_token.is_none());
    assert!(creds.expires_at.is_none());
    assert!(creds.scopes.is_empty());
}

#[test]
fn test_credentials_serialization_roundtrip() {
    let expires = Utc::now() + Duration::hours(6);
    let creds = Credentials {
        access_token: "tok_roundtrip".to_owned(),
        refresh_token: Some("ref_roundtrip".to_owned()),
        expires_at: Some(expires),
        scopes: vec!["read".to_owned(), "write".to_owned()],
    };

    let json = serde_json::to_string(&creds).expect("Credentials should serialize to JSON");
    let roundtripped: Credentials =
        serde_json::from_str(&json).expect("JSON should deserialize back to Credentials");

    assert_eq!(roundtripped.access_token, creds.access_token);
    assert_eq!(roundtripped.refresh_token, creds.refresh_token);
    assert_eq!(roundtripped.scopes, creds.scopes);
}

#[test]
fn test_continuous_metric_batch_with_points() {
    let now = Utc::now();
    let batch = ContinuousMetricBatch {
        series_type_id: 1,
        points: vec![
            (now, 72.0),
            (now + Duration::minutes(1), 73.5),
            (now + Duration::minutes(2), 71.0),
        ],
    };

    assert_eq!(batch.series_type_id, 1);
    assert_eq!(batch.points.len(), 3);
    assert!((batch.points[0].1 - 72.0).abs() < f64::EPSILON);
    assert!((batch.points[1].1 - 73.5).abs() < f64::EPSILON);
}

#[test]
fn test_continuous_metric_batch_empty_points() {
    let batch = ContinuousMetricBatch {
        series_type_id: 5,
        points: vec![],
    };

    assert_eq!(batch.series_type_id, 5);
    assert!(batch.points.is_empty());
}

#[test]
fn test_continuous_metric_batch_series_type_ids() {
    // Heart rate series
    let hr_batch = ContinuousMetricBatch {
        series_type_id: 1,
        points: vec![(Utc::now(), 72.0)],
    };
    assert_eq!(hr_batch.series_type_id, 1);

    // Steps series
    let steps_batch = ContinuousMetricBatch {
        series_type_id: 2,
        points: vec![(Utc::now(), 150.0)],
    };
    assert_eq!(steps_batch.series_type_id, 2);

    // Respiratory rate series
    let rr_batch = ContinuousMetricBatch {
        series_type_id: 10,
        points: vec![(Utc::now(), 14.5)],
    };
    assert_eq!(rr_batch.series_type_id, 10);
}

#[test]
fn test_continuous_metric_batch_serialization_roundtrip() {
    let now = Utc::now();
    let batch = ContinuousMetricBatch {
        series_type_id: 42,
        points: vec![(now, 98.6), (now + Duration::seconds(30), 98.7)],
    };

    let json = serde_json::to_string(&batch).expect("ContinuousMetricBatch should serialize");
    let roundtripped: ContinuousMetricBatch =
        serde_json::from_str(&json).expect("JSON should deserialize back to ContinuousMetricBatch");

    assert_eq!(roundtripped.series_type_id, batch.series_type_id);
    assert_eq!(roundtripped.points.len(), batch.points.len());
}
