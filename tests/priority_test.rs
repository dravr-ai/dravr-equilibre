// ABOUTME: Tests for the device and provider priority deduplication system
// ABOUTME: Validates priority ordering, duplicate resolution, and edge cases
//
// SPDX-License-Identifier: Apache-2.0
// Copyright (c) 2026 dravr.ai

use dravr_equilibre::data_source::{DataSource, DeviceType};
use dravr_equilibre::priority::{resolve_duplicate, DevicePriority, ProviderPriority};

#[test]
fn test_device_priority_ordering() {
    assert_eq!(DevicePriority::priority(&DeviceType::Watch), 1);
    assert_eq!(DevicePriority::priority(&DeviceType::Band), 2);
    assert_eq!(DevicePriority::priority(&DeviceType::Ring), 3);
    assert_eq!(DevicePriority::priority(&DeviceType::Phone), 4);
    assert_eq!(DevicePriority::priority(&DeviceType::Scale), 5);
    assert_eq!(DevicePriority::priority(&DeviceType::Unknown), 99);
}

#[test]
fn test_device_priority_watch_beats_phone() {
    let watch_priority = DevicePriority::priority(&DeviceType::Watch);
    let phone_priority = DevicePriority::priority(&DeviceType::Phone);
    assert!(watch_priority < phone_priority);
}

#[test]
fn test_device_priority_band_beats_scale() {
    let band_priority = DevicePriority::priority(&DeviceType::Band);
    let scale_priority = DevicePriority::priority(&DeviceType::Scale);
    assert!(band_priority < scale_priority);
}

#[test]
fn test_device_priority_ring_beats_phone() {
    let ring_priority = DevicePriority::priority(&DeviceType::Ring);
    let phone_priority = DevicePriority::priority(&DeviceType::Phone);
    assert!(ring_priority < phone_priority);
}

#[test]
fn test_provider_priority_known_providers() {
    assert_eq!(ProviderPriority::priority("apple"), 1);
    assert_eq!(ProviderPriority::priority("garmin"), 2);
    assert_eq!(ProviderPriority::priority("polar"), 3);
    assert_eq!(ProviderPriority::priority("suunto"), 4);
    assert_eq!(ProviderPriority::priority("whoop"), 5);
    assert_eq!(ProviderPriority::priority("oura"), 6);
    assert_eq!(ProviderPriority::priority("fitbit"), 7);
    assert_eq!(ProviderPriority::priority("coros"), 8);
}

#[test]
fn test_provider_priority_unknown_provider() {
    assert_eq!(ProviderPriority::priority("some_unknown_provider"), 100);
    assert_eq!(ProviderPriority::priority(""), 100);
}

#[test]
fn test_provider_priority_case_insensitive() {
    assert_eq!(ProviderPriority::priority("Apple"), 1);
    assert_eq!(ProviderPriority::priority("GARMIN"), 2);
    assert_eq!(ProviderPriority::priority("Polar"), 3);
}

#[test]
fn test_provider_priority_apple_beats_garmin() {
    let apple_priority = ProviderPriority::priority("apple");
    let garmin_priority = ProviderPriority::priority("garmin");
    assert!(apple_priority < garmin_priority);
}

fn make_data_source(id: &str, provider: &str, device_type: DeviceType) -> DataSource {
    DataSource {
        id: id.to_owned(),
        user_id: "user-test".to_owned(),
        provider: provider.to_owned(),
        device_model: None,
        software_version: None,
        source: None,
        device_type,
        original_source_name: None,
    }
}

#[test]
fn test_resolve_duplicate_prefers_watch_over_phone() {
    let watch = make_data_source("ds-watch", "garmin", DeviceType::Watch);
    let phone = make_data_source("ds-phone", "garmin", DeviceType::Phone);

    let winner = resolve_duplicate(&watch, &phone);
    assert_eq!(winner.id, "ds-watch");

    // Order should not matter
    let winner = resolve_duplicate(&phone, &watch);
    assert_eq!(winner.id, "ds-watch");
}

#[test]
fn test_resolve_duplicate_prefers_band_over_phone() {
    let band = make_data_source("ds-band", "fitbit", DeviceType::Band);
    let phone = make_data_source("ds-phone", "fitbit", DeviceType::Phone);

    let winner = resolve_duplicate(&band, &phone);
    assert_eq!(winner.id, "ds-band");
}

#[test]
fn test_resolve_duplicate_same_device_type_uses_provider_priority() {
    let apple_watch = make_data_source("ds-apple", "apple", DeviceType::Watch);
    let garmin_watch = make_data_source("ds-garmin", "garmin", DeviceType::Watch);

    let winner = resolve_duplicate(&apple_watch, &garmin_watch);
    assert_eq!(winner.id, "ds-apple");

    // Order should not matter
    let winner = resolve_duplicate(&garmin_watch, &apple_watch);
    assert_eq!(winner.id, "ds-apple");
}

#[test]
fn test_resolve_duplicate_device_priority_trumps_provider() {
    // Garmin watch (device=1, provider=2) vs Apple phone (device=4, provider=1)
    // Device priority should win: watch beats phone
    let garmin_watch = make_data_source("ds-garmin-watch", "garmin", DeviceType::Watch);
    let apple_phone = make_data_source("ds-apple-phone", "apple", DeviceType::Phone);

    let winner = resolve_duplicate(&garmin_watch, &apple_phone);
    assert_eq!(winner.id, "ds-garmin-watch");
}

#[test]
fn test_resolve_duplicate_identical_sources() {
    let a = make_data_source("ds-a", "garmin", DeviceType::Watch);
    let b = make_data_source("ds-b", "garmin", DeviceType::Watch);

    // Same device type and provider - should return first argument
    let winner = resolve_duplicate(&a, &b);
    assert_eq!(winner.id, "ds-a");
}

#[test]
fn test_resolve_duplicate_unknown_devices() {
    let unknown_a = make_data_source("ds-unknown-a", "apple", DeviceType::Unknown);
    let unknown_b = make_data_source("ds-unknown-b", "garmin", DeviceType::Unknown);

    // Both unknown device types - fall through to provider priority
    let winner = resolve_duplicate(&unknown_a, &unknown_b);
    assert_eq!(winner.id, "ds-unknown-a"); // Apple has higher provider priority
}

#[test]
fn test_resolve_duplicate_ring_vs_scale() {
    let ring = make_data_source("ds-ring", "oura", DeviceType::Ring);
    let scale = make_data_source("ds-scale", "fitbit", DeviceType::Scale);

    let winner = resolve_duplicate(&ring, &scale);
    assert_eq!(winner.id, "ds-ring");
}
