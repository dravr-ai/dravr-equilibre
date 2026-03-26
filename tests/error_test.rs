// ABOUTME: Tests for structured error types including display formatting and trait implementations
// ABOUTME: Validates all EquilibreError variants, Display output, and std::error::Error compliance
//
// SPDX-License-Identifier: Apache-2.0
// Copyright (c) 2026 dravr.ai

use dravr_equilibre::error::EquilibreError;

#[test]
fn test_missing_field_display_contains_field_name() {
    let err = EquilibreError::missing_field("user_id");
    let display = err.to_string();
    assert!(
        display.contains("user_id"),
        "MissingField display should contain field name, got: {display}"
    );
}

#[test]
fn test_out_of_range_display_contains_all_parts() {
    let err = EquilibreError::out_of_range("heart_rate", 300.0, 0.0, 250.0);
    let display = err.to_string();
    assert!(
        display.contains("heart_rate"),
        "OutOfRange display should contain field name, got: {display}"
    );
    assert!(
        display.contains("300"),
        "OutOfRange display should contain value, got: {display}"
    );
    assert!(
        display.contains('0'),
        "OutOfRange display should contain min, got: {display}"
    );
    assert!(
        display.contains("250"),
        "OutOfRange display should contain max, got: {display}"
    );
}

#[test]
fn test_stages_serialization_display_contains_reason() {
    let err = EquilibreError::stages_serialization("invalid JSON structure");
    let display = err.to_string();
    assert!(
        display.contains("invalid JSON structure"),
        "StagesSerialization display should contain reason, got: {display}"
    );
}

#[test]
fn test_invalid_time_range_display() {
    let err = EquilibreError::InvalidTimeRange;
    let display = err.to_string();
    assert!(
        display.contains("start must be before end"),
        "InvalidTimeRange display should explain the constraint, got: {display}"
    );
}

#[test]
fn test_provider_not_supported_display_contains_provider_and_capability() {
    let err = EquilibreError::provider_not_supported("strava", "continuous_data");
    let display = err.to_string();
    assert!(
        display.contains("strava"),
        "ProviderNotSupported display should contain provider name, got: {display}"
    );
    assert!(
        display.contains("continuous_data"),
        "ProviderNotSupported display should contain capability, got: {display}"
    );
}

#[test]
fn test_duplicate_data_source_display_contains_source_id() {
    let err = EquilibreError::duplicate_data_source("ds-001");
    let display = err.to_string();
    assert!(
        display.contains("ds-001"),
        "DuplicateDataSource display should contain source_id, got: {display}"
    );
}

#[test]
fn test_all_errors_implement_std_error() {
    // Verify that EquilibreError can be used as a trait object of std::error::Error
    fn assert_is_std_error<E: std::error::Error>(_e: &E) {}

    let errors: Vec<EquilibreError> = vec![
        EquilibreError::missing_field("test"),
        EquilibreError::out_of_range("test", 0.0, 0.0, 100.0),
        EquilibreError::stages_serialization("test"),
        EquilibreError::InvalidTimeRange,
        EquilibreError::provider_not_supported("test", "test"),
        EquilibreError::duplicate_data_source("test"),
    ];

    for err in &errors {
        assert_is_std_error(err);
    }
}

#[test]
fn test_all_errors_implement_display() {
    fn assert_has_display<D: std::fmt::Display>(d: &D) -> String {
        d.to_string()
    }

    let errors: Vec<EquilibreError> = vec![
        EquilibreError::missing_field("field_x"),
        EquilibreError::out_of_range("field_y", 5.0, 0.0, 3.0),
        EquilibreError::stages_serialization("bad data"),
        EquilibreError::InvalidTimeRange,
        EquilibreError::provider_not_supported("provider_z", "oauth"),
        EquilibreError::duplicate_data_source("ds-dup"),
    ];

    for err in &errors {
        let display = assert_has_display(err);
        assert!(
            !display.is_empty(),
            "Display should produce non-empty string for {err:?}"
        );
    }
}

#[test]
fn test_error_debug_format() {
    let err = EquilibreError::missing_field("email");
    let debug = format!("{err:?}");
    assert!(
        !debug.is_empty(),
        "Debug format should produce non-empty output"
    );
    assert!(
        debug.contains("MissingField"),
        "Debug format should contain variant name, got: {debug}"
    );
}

#[test]
fn test_serialization_error_from_serde_json() {
    // Create an invalid JSON parse to trigger serde_json::Error conversion
    let parse_result: Result<serde_json::Value, _> = serde_json::from_str("not valid json");
    let serde_err = parse_result.expect_err("invalid JSON should produce parse error");

    let equilibre_err: EquilibreError = serde_err.into();
    let display = equilibre_err.to_string();
    assert!(
        display.contains("serialization error"),
        "Serialization variant should display with prefix, got: {display}"
    );
}
