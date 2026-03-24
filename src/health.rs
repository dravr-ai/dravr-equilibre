// ABOUTME: Health metrics model for body composition and vital signs
// ABOUTME: Weight, body fat, BMI, blood pressure, and glucose tracking from health providers
//
// SPDX-License-Identifier: Apache-2.0
// Copyright (c) 2026 dravr.ai

//! Body composition and vital sign metrics.
//!
//! [`StoredHealthMetrics`] captures periodic health measurements including
//! body composition (weight, body fat, muscle mass), cardiovascular vitals
//! (blood pressure), and metabolic markers (blood glucose).

use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};

/// Periodic health and body composition metrics.
///
/// Designed for database persistence with denormalized metrics
/// for trend analysis and health dashboard queries.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoredHealthMetrics {
    /// Unique identifier.
    pub id: String,
    /// User who recorded these metrics.
    pub user_id: String,
    /// Data source that produced these metrics.
    pub data_source_id: String,
    /// Calendar date for these metrics.
    pub date: NaiveDate,
    /// Body weight in kilograms.
    pub weight_kg: Option<f64>,
    /// Body fat percentage.
    pub body_fat_pct: Option<f64>,
    /// Muscle mass in kilograms.
    pub muscle_mass_kg: Option<f64>,
    /// Body mass index.
    pub bmi: Option<f64>,
    /// Bone mass in kilograms.
    pub bone_mass_kg: Option<f64>,
    /// Body water percentage.
    pub water_pct: Option<f64>,
    /// Systolic blood pressure (mmHg).
    pub systolic_bp: Option<u32>,
    /// Diastolic blood pressure (mmHg).
    pub diastolic_bp: Option<u32>,
    /// Blood glucose level (mg/dL).
    pub blood_glucose: Option<f64>,
    /// Provider-specific source name.
    pub source_name: String,
    /// Timestamp when these metrics were recorded.
    pub recorded_at: DateTime<Utc>,
}
