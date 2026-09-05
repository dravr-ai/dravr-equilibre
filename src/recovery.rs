// ABOUTME: Recovery metrics model for daily readiness and recovery scoring
// ABOUTME: HRV, resting HR, stress, and composite recovery scores from wearable providers
//
// SPDX-License-Identifier: Apache-2.0
// Copyright (c) 2026 dravr.ai

//! Daily recovery and readiness metrics.
//!
//! [`StoredRecoveryMetrics`] captures the daily recovery state reported by
//! wearable devices including HRV, resting heart rate, stress levels,
//! and composite recovery scores.

use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};

/// Daily recovery metrics from a wearable device.
///
/// Designed for database persistence with denormalized metrics
/// for fast dashboard queries.
#[derive(Debug, Clone, Serialize, Deserialize, schemars::JsonSchema)]
pub struct StoredRecoveryMetrics {
    /// Unique identifier.
    pub id: String,
    /// User who recorded these metrics.
    pub user_id: String,
    /// Data source that produced these metrics.
    pub data_source_id: String,
    /// Calendar date for these metrics.
    pub date: NaiveDate,
    /// Composite recovery score (provider-specific, 0-100).
    pub recovery_score: Option<u32>,
    /// Readiness score (provider-specific, 0-100).
    pub readiness_score: Option<u32>,
    /// Heart rate variability in milliseconds.
    pub hrv_ms: Option<f64>,
    /// HRV as RMSSD (root mean square of successive differences).
    pub hrv_rmssd: Option<f64>,
    /// Resting heart rate (bpm).
    pub resting_heart_rate: Option<u32>,
    /// Stress level score (provider-specific, 0-100).
    pub stress_score: Option<u32>,
    /// Body battery or energy level (provider-specific, 0-100).
    pub body_battery: Option<u32>,
    /// Blood oxygen saturation percentage (`SpO2`).
    pub spo2: Option<f64>,
    /// Respiratory rate (breaths per minute).
    pub respiratory_rate: Option<f64>,
    /// Skin temperature deviation from baseline (degrees Celsius).
    pub skin_temp_deviation: Option<f64>,
    /// Provider-specific source name.
    pub source_name: String,
    /// Timestamp when these metrics were recorded.
    pub recorded_at: DateTime<Utc>,
}
