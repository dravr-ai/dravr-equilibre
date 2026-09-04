// ABOUTME: Provider-sourced data models for health data synchronization
// ABOUTME: ContinuousMetricBatch time series returned by continuous-monitoring providers
//
// SPDX-License-Identifier: Apache-2.0
// Copyright (c) 2026 dravr.ai

//! Provider-sourced data models for health data synchronization.
//!
//! Providers deliver continuous monitoring streams (heart rate, steps,
//! respiratory rate) as [`ContinuousMetricBatch`] time series, one batch
//! per metric series type.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// A batch of continuous metric data points from a provider.
///
/// Each batch represents a time series for a specific metric type
/// (e.g., heart rate, steps, respiratory rate).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContinuousMetricBatch {
    /// Identifier for the metric series type.
    pub series_type_id: u32,
    /// Time-value pairs for the metric.
    pub points: Vec<(DateTime<Utc>, f64)>,
}
