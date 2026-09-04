// ABOUTME: Tests for provider-sourced data models
// ABOUTME: Validates ContinuousMetricBatch construction and serialization round-trips
//
// SPDX-License-Identifier: Apache-2.0
// Copyright (c) 2026 dravr.ai

#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::str_to_string
)]

use chrono::{Duration, Utc};
use dravr_equilibre::provider::ContinuousMetricBatch;

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
