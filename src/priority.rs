// ABOUTME: Deduplication priority system for devices and providers
// ABOUTME: Determines which data source takes precedence when duplicates are detected
//
// SPDX-License-Identifier: Apache-2.0
// Copyright (c) 2026 dravr.ai

//! Priority system for deduplicating overlapping health data.
//!
//! When multiple devices or providers report the same data (e.g., both an Apple Watch
//! and an iPhone record the same workout), the priority system determines which
//! source is authoritative.
//!
//! **Device priority** (lower = higher priority):
//! Watch (1) > Band (2) > Ring (3) > Phone (4) > Scale (5) > Unknown (99)
//!
//! **Provider priority** (lower = higher priority):
//! apple (1) > garmin (2) > polar (3) > suunto (4) > whoop (5) > oura (6) > fitbit (7) > coros (8)

use std::cmp::Ordering;

use crate::data_source::{DataSource, DeviceType};

/// Priority ranking for device types.
///
/// Wearable devices with continuous sensor contact (watches, bands, rings)
/// are preferred over phones and scales.
pub struct DevicePriority;

impl DevicePriority {
    /// Return the priority value for a device type.
    /// Lower values indicate higher priority.
    pub fn priority(device_type: &DeviceType) -> u8 {
        match device_type {
            DeviceType::Watch => 1,
            DeviceType::Band => 2,
            DeviceType::Ring => 3,
            DeviceType::Phone => 4,
            DeviceType::Scale => 5,
            DeviceType::Unknown => 99,
        }
    }
}

/// Priority ranking for data providers.
///
/// Providers with tighter hardware-software integration and richer sensor
/// data are ranked higher.
pub struct ProviderPriority;

impl ProviderPriority {
    /// Return the priority value for a provider name.
    /// Lower values indicate higher priority. Unknown providers get priority 100.
    pub fn priority(provider: &str) -> u8 {
        match provider.to_lowercase().as_str() {
            "apple" => 1,
            "garmin" => 2,
            "polar" => 3,
            "suunto" => 4,
            "whoop" => 5,
            "oura" => 6,
            "fitbit" => 7,
            "coros" => 8,
            _ => 100,
        }
    }
}

/// Resolve a duplicate between two data sources by choosing the higher-priority one.
///
/// Uses device type priority first, then falls back to provider priority
/// if device types have equal priority.
///
/// Returns a reference to the preferred data source.
pub fn resolve_duplicate<'a>(a: &'a DataSource, b: &'a DataSource) -> &'a DataSource {
    let device_a = DevicePriority::priority(&a.device_type);
    let device_b = DevicePriority::priority(&b.device_type);

    match device_a.cmp(&device_b) {
        Ordering::Less => a,
        Ordering::Greater => b,
        Ordering::Equal => {
            let provider_a = ProviderPriority::priority(&a.provider);
            let provider_b = ProviderPriority::priority(&b.provider);
            if provider_a <= provider_b {
                a
            } else {
                b
            }
        }
    }
}
