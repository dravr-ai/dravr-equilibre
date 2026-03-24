// ABOUTME: Workout details model extending EventRecord with exercise-specific metrics
// ABOUTME: Heart rate, power, distance, elevation, and energy data for physical activities
//
// SPDX-License-Identifier: Apache-2.0
// Copyright (c) 2026 dravr.ai

//! Workout details for physical activity events.
//!
//! [`WorkoutDetails`] extends [`EventRecord`](crate::event::EventRecord) with
//! exercise-specific metrics including heart rate zones, power output,
//! distance, elevation, and energy expenditure.

use serde::{Deserialize, Serialize};

use crate::event::EventRecord;

/// Detailed workout metrics extending the base event record.
///
/// Contains physiological and performance metrics captured during
/// a workout session by wearable devices.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkoutDetails {
    /// Base event data (timing, source, category).
    pub event: EventRecord,
    /// Minimum heart rate observed during the workout (bpm).
    pub heart_rate_min: Option<u32>,
    /// Maximum heart rate observed during the workout (bpm).
    pub heart_rate_max: Option<u32>,
    /// Average heart rate during the workout (bpm).
    pub heart_rate_avg: Option<f64>,
    /// Total active energy burned (kilocalories).
    pub energy_burned: Option<f64>,
    /// Total distance covered (meters).
    pub distance: Option<f64>,
    /// Total step count during the workout.
    pub steps_count: Option<u32>,
    /// Maximum speed recorded (meters per second).
    pub max_speed: Option<f64>,
    /// Average speed during the workout (meters per second).
    pub average_speed: Option<f64>,
    /// Maximum power output (watts).
    pub max_watts: Option<u32>,
    /// Average power output (watts).
    pub average_watts: Option<u32>,
    /// Time spent moving, excluding pauses (seconds).
    pub moving_time_seconds: Option<u32>,
    /// Total elevation gain (meters).
    pub total_elevation_gain: Option<f64>,
    /// Highest elevation reached (meters).
    pub elev_high: Option<f64>,
    /// Lowest elevation reached (meters).
    pub elev_low: Option<f64>,
}
