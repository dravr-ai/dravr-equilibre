// ABOUTME: Data source and device tracking models for health data providers
// ABOUTME: Tracks device model, type, software version, and original source metadata
//
// SPDX-License-Identifier: Apache-2.0
// Copyright (c) 2026 dravr.ai

//! Data source tracking for health data providers.
//!
//! Each [`DataSource`] represents a specific device or app that produces health data.
//! The [`DeviceType`] enum classifies the hardware form factor for deduplication
//! priority ordering.

use std::fmt::{self, Display};

use serde::{Deserialize, Serialize};

/// Classification of wearable device form factors.
///
/// Used by [`DevicePriority`](crate::priority::DevicePriority) to determine
/// which data source takes precedence during deduplication.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DeviceType {
    /// Smartwatch (e.g., Apple Watch, Garmin Forerunner)
    Watch,
    /// Fitness band (e.g., Fitbit Charge)
    Band,
    /// Smartphone (e.g., iPhone, Android phone)
    Phone,
    /// Smart ring (e.g., Oura Ring)
    Ring,
    /// Smart scale (e.g., Withings Body)
    Scale,
    /// Unknown or unclassified device
    Unknown,
}

impl Display for DeviceType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Watch => write!(f, "Watch"),
            Self::Band => write!(f, "Band"),
            Self::Phone => write!(f, "Phone"),
            Self::Ring => write!(f, "Ring"),
            Self::Scale => write!(f, "Scale"),
            Self::Unknown => write!(f, "Unknown"),
        }
    }
}

/// A data source representing a specific device or application providing health data.
///
/// Tracks the provider, device hardware, software version, and original source
/// metadata for data lineage and deduplication.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataSource {
    /// Unique identifier for this data source.
    pub id: String,
    /// User who owns this data source.
    pub user_id: String,
    /// Provider name (e.g., "garmin", "apple", "strava").
    pub provider: String,
    /// Device model name (e.g., "Apple Watch Series 9", "Forerunner 965").
    pub device_model: Option<String>,
    /// Software or firmware version.
    pub software_version: Option<String>,
    /// Platform source identifier (e.g., "com.apple.health").
    pub source: Option<String>,
    /// Form factor classification.
    pub device_type: DeviceType,
    /// Original source name as reported by the provider.
    pub original_source_name: Option<String>,
}
