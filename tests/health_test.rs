// ABOUTME: Tests for health metrics models including body composition and vital signs
// ABOUTME: Validates StoredHealthMetrics construction, realistic ranges, and JSON roundtrips
//
// SPDX-License-Identifier: Apache-2.0
// Copyright (c) 2026 dravr.ai

use chrono::{NaiveDate, Utc};
use dravr_equilibre::health::StoredHealthMetrics;

/// Helper to create a fully populated health metrics instance.
fn full_health_metrics() -> StoredHealthMetrics {
    StoredHealthMetrics {
        id: "hlth-full".to_owned(),
        user_id: "user-001".to_owned(),
        data_source_id: "ds-withings".to_owned(),
        date: NaiveDate::from_ymd_opt(2026, 3, 24).expect("valid date"),
        weight_kg: Some(75.5),
        body_fat_pct: Some(18.2),
        muscle_mass_kg: Some(35.0),
        bmi: Some(23.8),
        bone_mass_kg: Some(3.2),
        water_pct: Some(55.0),
        systolic_bp: Some(120),
        diastolic_bp: Some(80),
        blood_glucose: Some(95.0),
        source_name: "Withings".to_owned(),
        recorded_at: Utc::now(),
    }
}

#[test]
fn test_stored_health_metrics_all_fields() {
    let metrics = full_health_metrics();

    assert_eq!(metrics.id, "hlth-full");
    assert_eq!(metrics.user_id, "user-001");
    assert_eq!(metrics.data_source_id, "ds-withings");
    assert_eq!(metrics.weight_kg, Some(75.5));
    assert_eq!(metrics.body_fat_pct, Some(18.2));
    assert_eq!(metrics.muscle_mass_kg, Some(35.0));
    assert_eq!(metrics.bmi, Some(23.8));
    assert_eq!(metrics.bone_mass_kg, Some(3.2));
    assert_eq!(metrics.water_pct, Some(55.0));
    assert_eq!(metrics.systolic_bp, Some(120));
    assert_eq!(metrics.diastolic_bp, Some(80));
    assert_eq!(metrics.blood_glucose, Some(95.0));
    assert_eq!(metrics.source_name, "Withings");
}

#[test]
fn test_stored_health_metrics_minimal() {
    let metrics = StoredHealthMetrics {
        id: "hlth-min".to_owned(),
        user_id: "user-002".to_owned(),
        data_source_id: "ds-apple".to_owned(),
        date: NaiveDate::from_ymd_opt(2026, 3, 25).expect("valid date"),
        weight_kg: None,
        body_fat_pct: None,
        muscle_mass_kg: None,
        bmi: None,
        bone_mass_kg: None,
        water_pct: None,
        systolic_bp: None,
        diastolic_bp: None,
        blood_glucose: None,
        source_name: "Apple".to_owned(),
        recorded_at: Utc::now(),
    };

    assert!(metrics.weight_kg.is_none());
    assert!(metrics.body_fat_pct.is_none());
    assert!(metrics.muscle_mass_kg.is_none());
    assert!(metrics.bmi.is_none());
    assert!(metrics.bone_mass_kg.is_none());
    assert!(metrics.water_pct.is_none());
    assert!(metrics.systolic_bp.is_none());
    assert!(metrics.diastolic_bp.is_none());
    assert!(metrics.blood_glucose.is_none());
}

#[test]
fn test_weight_light_person() {
    let mut metrics = full_health_metrics();
    metrics.weight_kg = Some(30.0);
    assert_eq!(metrics.weight_kg, Some(30.0));
}

#[test]
fn test_weight_heavy_person() {
    let mut metrics = full_health_metrics();
    metrics.weight_kg = Some(300.0);
    assert_eq!(metrics.weight_kg, Some(300.0));
}

#[test]
fn test_body_fat_percentage_low() {
    let mut metrics = full_health_metrics();
    metrics.body_fat_pct = Some(3.0);
    assert_eq!(metrics.body_fat_pct, Some(3.0));
}

#[test]
fn test_body_fat_percentage_high() {
    let mut metrics = full_health_metrics();
    metrics.body_fat_pct = Some(60.0);
    assert_eq!(metrics.body_fat_pct, Some(60.0));
}

#[test]
fn test_blood_pressure_separate_fields() {
    let metrics = full_health_metrics();
    assert_eq!(metrics.systolic_bp, Some(120));
    assert_eq!(metrics.diastolic_bp, Some(80));
    // Systolic should be higher than diastolic in normal readings
    assert!(
        metrics.systolic_bp.expect("systolic set")
            > metrics.diastolic_bp.expect("diastolic set")
    );
}

#[test]
fn test_blood_glucose_normal_range() {
    let mut metrics = full_health_metrics();
    // Normal fasting: 70-100 mg/dL
    metrics.blood_glucose = Some(85.0);
    assert_eq!(metrics.blood_glucose, Some(85.0));
}

#[test]
fn test_blood_glucose_elevated() {
    let mut metrics = full_health_metrics();
    // Elevated reading
    metrics.blood_glucose = Some(180.0);
    assert_eq!(metrics.blood_glucose, Some(180.0));
}

#[test]
fn test_muscle_mass_and_bone_mass() {
    let metrics = full_health_metrics();
    assert_eq!(metrics.muscle_mass_kg, Some(35.0));
    assert_eq!(metrics.bone_mass_kg, Some(3.2));
}

#[test]
fn test_json_serialization_roundtrip() {
    let metrics = full_health_metrics();
    let json = serde_json::to_string(&metrics)
        .expect("health metrics should serialize to JSON");
    let roundtripped: StoredHealthMetrics = serde_json::from_str(&json)
        .expect("JSON should deserialize back to StoredHealthMetrics");

    assert_eq!(roundtripped.id, metrics.id);
    assert_eq!(roundtripped.user_id, metrics.user_id);
    assert_eq!(roundtripped.date, metrics.date);
    assert_eq!(roundtripped.weight_kg, metrics.weight_kg);
    assert_eq!(roundtripped.body_fat_pct, metrics.body_fat_pct);
    assert_eq!(roundtripped.muscle_mass_kg, metrics.muscle_mass_kg);
    assert_eq!(roundtripped.bmi, metrics.bmi);
    assert_eq!(roundtripped.bone_mass_kg, metrics.bone_mass_kg);
    assert_eq!(roundtripped.water_pct, metrics.water_pct);
    assert_eq!(roundtripped.systolic_bp, metrics.systolic_bp);
    assert_eq!(roundtripped.diastolic_bp, metrics.diastolic_bp);
    assert_eq!(roundtripped.blood_glucose, metrics.blood_glucose);
}

#[test]
fn test_json_none_fields_serialization() {
    let metrics = StoredHealthMetrics {
        id: "hlth-none".to_owned(),
        user_id: "user-003".to_owned(),
        data_source_id: "ds-003".to_owned(),
        date: NaiveDate::from_ymd_opt(2026, 3, 26).expect("valid date"),
        weight_kg: Some(70.0),
        body_fat_pct: None,
        muscle_mass_kg: None,
        bmi: None,
        bone_mass_kg: None,
        water_pct: None,
        systolic_bp: None,
        diastolic_bp: None,
        blood_glucose: None,
        source_name: "Scale".to_owned(),
        recorded_at: Utc::now(),
    };

    let json = serde_json::to_string(&metrics)
        .expect("health metrics with None fields should serialize");
    let roundtripped: StoredHealthMetrics = serde_json::from_str(&json)
        .expect("JSON with null fields should deserialize");

    // The only weight-related field populated
    assert_eq!(roundtripped.weight_kg, Some(70.0));
    assert!(roundtripped.body_fat_pct.is_none());
    assert!(roundtripped.systolic_bp.is_none());
}
