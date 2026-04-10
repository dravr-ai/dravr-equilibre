// ABOUTME: Base event record model for polymorphic health events
// ABOUTME: Shared fields for workouts, sleep sessions, and other time-bounded events
//
// SPDX-License-Identifier: Apache-2.0
// Copyright (c) 2026 dravr.ai

//! Base event model for time-bounded health events.
//!
//! [`EventRecord`] provides the common fields shared by all event types
//! (workouts, sleep sessions, etc.). Specific event types extend this
//! base with domain-specific fields.

use std::fmt::{self, Display};

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Category of a health event.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EventCategory {
    /// Physical activity or exercise session.
    Workout,
    /// Sleep session (full night or nap).
    Sleep,
}

impl Display for EventCategory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Workout => write!(f, "Workout"),
            Self::Sleep => write!(f, "Sleep"),
        }
    }
}

/// Base event record shared by all event types.
///
/// Contains the common metadata for any time-bounded health event:
/// identifiers, timing, source tracking, and categorization.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventRecord {
    /// Internal unique identifier.
    pub id: String,
    /// External identifier from the provider (e.g., Strava activity ID).
    pub external_id: Option<String>,
    /// Reference to the data source that produced this event.
    pub data_source_id: String,
    /// High-level event category.
    pub category: EventCategory,
    /// Specific event type within the category (e.g., "Run", "Cycling", "Deep Sleep").
    pub event_type: String,
    /// Name of the originating source/provider.
    pub source_name: String,
    /// Duration in seconds, if known.
    pub duration_seconds: Option<u32>,
    /// Event start time in UTC.
    pub start_datetime: DateTime<Utc>,
    /// Event end time in UTC.
    pub end_datetime: DateTime<Utc>,
    /// Timezone offset string (e.g., "+05:00", "-08:00").
    pub zone_offset: Option<String>,
}
