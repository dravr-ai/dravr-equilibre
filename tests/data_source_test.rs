// ABOUTME: Tests for DataSource and DeviceType models including serialization and classification
// ABOUTME: Validates device type variants, display formatting, and data source JSON roundtrips
//
// SPDX-License-Identifier: Apache-2.0
// Copyright (c) 2026 dravr.ai

use dravr_equilibre::data_source::{DataSource, DeviceType};

/// Helper to create a fully populated data source.
fn full_data_source(id: &str, provider: &str, device_type: DeviceType) -> DataSource {
    DataSource {
        id: id.to_owned(),
        user_id: "user-001".to_owned(),
        provider: provider.to_owned(),
        device_model: Some("Test Device".to_owned()),
        software_version: Some("1.0.0".to_owned()),
        source: Some("com.test.health".to_owned()),
        device_type,
        original_source_name: Some("Test Provider".to_owned()),
    }
}

/// Helper to create a minimal data source (all optional fields None).
fn minimal_data_source(id: &str, provider: &str, device_type: DeviceType) -> DataSource {
    DataSource {
        id: id.to_owned(),
        user_id: "user-001".to_owned(),
        provider: provider.to_owned(),
        device_model: None,
        software_version: None,
        source: None,
        device_type,
        original_source_name: None,
    }
}

#[test]
fn test_data_source_all_fields_populated() {
    let ds = full_data_source("ds-001", "garmin", DeviceType::Watch);

    assert_eq!(ds.id, "ds-001");
    assert_eq!(ds.user_id, "user-001");
    assert_eq!(ds.provider, "garmin");
    assert_eq!(ds.device_model.as_deref(), Some("Test Device"));
    assert_eq!(ds.software_version.as_deref(), Some("1.0.0"));
    assert_eq!(ds.source.as_deref(), Some("com.test.health"));
    assert_eq!(ds.device_type, DeviceType::Watch);
    assert_eq!(ds.original_source_name.as_deref(), Some("Test Provider"));
}

#[test]
fn test_data_source_minimal_fields() {
    let ds = minimal_data_source("ds-min", "strava", DeviceType::Phone);

    assert_eq!(ds.id, "ds-min");
    assert_eq!(ds.provider, "strava");
    assert!(ds.device_model.is_none());
    assert!(ds.software_version.is_none());
    assert!(ds.source.is_none());
    assert!(ds.original_source_name.is_none());
}

#[test]
fn test_device_type_watch() {
    let dt = DeviceType::Watch;
    assert_eq!(dt, DeviceType::Watch);
}

#[test]
fn test_device_type_band() {
    let dt = DeviceType::Band;
    assert_eq!(dt, DeviceType::Band);
}

#[test]
fn test_device_type_phone() {
    let dt = DeviceType::Phone;
    assert_eq!(dt, DeviceType::Phone);
}

#[test]
fn test_device_type_ring() {
    let dt = DeviceType::Ring;
    assert_eq!(dt, DeviceType::Ring);
}

#[test]
fn test_device_type_scale() {
    let dt = DeviceType::Scale;
    assert_eq!(dt, DeviceType::Scale);
}

#[test]
fn test_device_type_unknown() {
    let dt = DeviceType::Unknown;
    assert_eq!(dt, DeviceType::Unknown);
}

#[test]
fn test_device_type_serialization_all_variants() {
    let cases = [
        (DeviceType::Watch, "\"Watch\""),
        (DeviceType::Band, "\"Band\""),
        (DeviceType::Phone, "\"Phone\""),
        (DeviceType::Ring, "\"Ring\""),
        (DeviceType::Scale, "\"Scale\""),
        (DeviceType::Unknown, "\"Unknown\""),
    ];

    for (variant, expected) in cases {
        let json = serde_json::to_string(&variant).expect("DeviceType should serialize");
        assert_eq!(json, expected, "Serialization mismatch for {variant:?}");
    }
}

#[test]
fn test_device_type_deserialization_all_variants() {
    let cases = [
        ("\"Watch\"", DeviceType::Watch),
        ("\"Band\"", DeviceType::Band),
        ("\"Phone\"", DeviceType::Phone),
        ("\"Ring\"", DeviceType::Ring),
        ("\"Scale\"", DeviceType::Scale),
        ("\"Unknown\"", DeviceType::Unknown),
    ];

    for (json_str, expected) in cases {
        let deserialized: DeviceType =
            serde_json::from_str(json_str).expect("DeviceType should deserialize");
        assert_eq!(
            deserialized, expected,
            "Deserialization mismatch for {json_str}"
        );
    }
}

#[test]
fn test_device_type_display_all_variants() {
    assert_eq!(DeviceType::Watch.to_string(), "Watch");
    assert_eq!(DeviceType::Band.to_string(), "Band");
    assert_eq!(DeviceType::Phone.to_string(), "Phone");
    assert_eq!(DeviceType::Ring.to_string(), "Ring");
    assert_eq!(DeviceType::Scale.to_string(), "Scale");
    assert_eq!(DeviceType::Unknown.to_string(), "Unknown");
}

#[test]
fn test_data_source_with_each_device_type() {
    let device_types = [
        DeviceType::Watch,
        DeviceType::Band,
        DeviceType::Phone,
        DeviceType::Ring,
        DeviceType::Scale,
        DeviceType::Unknown,
    ];

    for (i, dt) in device_types.iter().enumerate() {
        let ds = full_data_source(&format!("ds-{i}"), "test", *dt);
        assert_eq!(ds.device_type, *dt);
    }
}

#[test]
fn test_data_source_json_roundtrip() {
    let ds = full_data_source("ds-rt", "garmin", DeviceType::Watch);
    let json = serde_json::to_string(&ds).expect("DataSource should serialize to JSON");
    let roundtripped: DataSource =
        serde_json::from_str(&json).expect("JSON should deserialize back to DataSource");

    assert_eq!(roundtripped.id, ds.id);
    assert_eq!(roundtripped.user_id, ds.user_id);
    assert_eq!(roundtripped.provider, ds.provider);
    assert_eq!(roundtripped.device_model, ds.device_model);
    assert_eq!(roundtripped.software_version, ds.software_version);
    assert_eq!(roundtripped.source, ds.source);
    assert_eq!(roundtripped.device_type, ds.device_type);
    assert_eq!(roundtripped.original_source_name, ds.original_source_name);
}

#[test]
fn test_multiple_data_sources_same_user_different_providers() {
    let garmin = full_data_source("ds-garmin", "garmin", DeviceType::Watch);
    let apple = full_data_source("ds-apple", "apple", DeviceType::Watch);

    assert_eq!(garmin.user_id, apple.user_id);
    assert_ne!(garmin.provider, apple.provider);
    assert_ne!(garmin.id, apple.id);
}

#[test]
fn test_multiple_data_sources_same_provider_different_devices() {
    let watch = DataSource {
        id: "ds-garmin-watch".to_owned(),
        user_id: "user-001".to_owned(),
        provider: "garmin".to_owned(),
        device_model: Some("Forerunner 965".to_owned()),
        software_version: None,
        source: None,
        device_type: DeviceType::Watch,
        original_source_name: None,
    };

    let scale = DataSource {
        id: "ds-garmin-scale".to_owned(),
        user_id: "user-001".to_owned(),
        provider: "garmin".to_owned(),
        device_model: Some("Index S2".to_owned()),
        software_version: None,
        source: None,
        device_type: DeviceType::Scale,
        original_source_name: None,
    };

    assert_eq!(watch.provider, scale.provider);
    assert_ne!(watch.device_type, scale.device_type);
    assert_ne!(watch.id, scale.id);
}
