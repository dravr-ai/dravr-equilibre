// ABOUTME: Sleep session models with stage tracking and nap detection
// ABOUTME: DB-ready StoredSleepSession with JSONB stages and SleepDetails for event extension
//
// SPDX-License-Identifier: Apache-2.0
// Copyright (c) 2026 dravr.ai

//! Sleep session tracking with stage-level detail.
//!
//! [`StoredSleepSession`] provides a database-ready model for sleep data
//! with JSONB-serializable stages. [`SleepDetails`] extends
//! [`EventRecord`](crate::event::EventRecord) for polymorphic event handling.

use std::fmt::{self, Display};

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::event::EventRecord;

/// Classification of a sleep stage.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, schemars::JsonSchema)]
pub enum SleepStageType {
    /// Awake period during sleep.
    Awake,
    /// Light sleep (NREM stages 1-2).
    Light,
    /// Deep sleep (NREM stage 3, slow-wave sleep).
    Deep,
    /// REM (rapid eye movement) sleep.
    Rem,
    /// Unknown or unclassified stage.
    Unknown,
}

impl Display for SleepStageType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Awake => write!(f, "Awake"),
            Self::Light => write!(f, "Light"),
            Self::Deep => write!(f, "Deep"),
            Self::Rem => write!(f, "REM"),
            Self::Unknown => write!(f, "Unknown"),
        }
    }
}

/// A single sleep stage within a session.
///
/// Records the type and duration of a contiguous sleep stage.
#[derive(Debug, Clone, Serialize, Deserialize, schemars::JsonSchema)]
pub struct SleepStage {
    /// Classification of this stage.
    pub stage_type: SleepStageType,
    /// Start time of this stage.
    pub start: DateTime<Utc>,
    /// End time of this stage.
    pub end: DateTime<Utc>,
    /// Duration of this stage in seconds.
    pub duration_seconds: u32,
}

/// Database-ready sleep session model.
///
/// Designed for persistence with JSONB-serializable stages and
/// nap detection. Includes summary metrics and optional HRV data.
#[derive(Debug, Clone, Serialize, Deserialize, schemars::JsonSchema)]
pub struct StoredSleepSession {
    /// Unique identifier.
    pub id: String,
    /// User who recorded this sleep session.
    pub user_id: String,
    /// Data source that produced this session.
    pub data_source_id: String,
    /// Whether this is a nap rather than a full night's sleep.
    pub is_nap: bool,
    /// Session start time.
    pub start_datetime: DateTime<Utc>,
    /// Session end time.
    pub end_datetime: DateTime<Utc>,
    /// Total sleep duration in seconds (excluding awake time).
    pub total_sleep_seconds: Option<u32>,
    /// Time spent in deep sleep (seconds).
    pub deep_sleep_seconds: Option<u32>,
    /// Time spent in light sleep (seconds).
    pub light_sleep_seconds: Option<u32>,
    /// Time spent in REM sleep (seconds).
    pub rem_sleep_seconds: Option<u32>,
    /// Time spent awake during the session (seconds).
    pub awake_seconds: Option<u32>,
    /// Sleep efficiency percentage (0.0-100.0).
    pub sleep_efficiency: Option<f64>,
    /// Average heart rate during sleep (bpm).
    pub avg_heart_rate: Option<f64>,
    /// Minimum heart rate during sleep (bpm).
    pub min_heart_rate: Option<u32>,
    /// Average HRV during sleep (ms).
    pub avg_hrv: Option<f64>,
    /// Sleep quality score (provider-specific, 0-100).
    pub sleep_score: Option<u32>,
    /// Detailed sleep stages (stored as JSONB in database).
    pub stages: Vec<SleepStage>,
    /// Provider-specific source name.
    pub source_name: String,
}

/// Sleep details extending the base event record.
///
/// Used for polymorphic event handling where sleep sessions
/// are treated as events alongside workouts.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SleepDetails {
    /// Base event data (timing, source, category).
    pub event: EventRecord,
    /// Whether this is a nap.
    pub is_nap: bool,
    /// Total sleep duration in seconds.
    pub total_sleep_seconds: Option<u32>,
    /// Sleep efficiency percentage.
    pub sleep_efficiency: Option<f64>,
    /// Sleep quality score.
    pub sleep_score: Option<u32>,
    /// Detailed sleep stages.
    pub stages: Vec<SleepStage>,
}
