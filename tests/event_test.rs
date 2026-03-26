// ABOUTME: Tests for event record models including workout and sleep event categories
// ABOUTME: Validates EventRecord construction, WorkoutDetails metrics, and JSON roundtrips
//
// SPDX-License-Identifier: Apache-2.0
// Copyright (c) 2026 dravr.ai

use chrono::{Duration, Utc};
use dravr_equilibre::event::{EventCategory, EventRecord};
use dravr_equilibre::workout::WorkoutDetails;

/// Helper to create a workout event record.
fn workout_event() -> EventRecord {
    let start = Utc::now();
    EventRecord {
        id: "evt-wkt-001".to_owned(),
        external_id: Some("strava-99999".to_owned()),
        data_source_id: "ds-garmin".to_owned(),
        category: EventCategory::Workout,
        event_type: "Run".to_owned(),
        source_name: "Garmin".to_owned(),
        duration_seconds: Some(3600),
        start_datetime: start,
        end_datetime: start + Duration::hours(1),
        zone_offset: Some("+02:00".to_owned()),
    }
}

/// Helper to create a sleep event record.
fn sleep_event() -> EventRecord {
    let start = Utc::now();
    EventRecord {
        id: "evt-slp-001".to_owned(),
        external_id: None,
        data_source_id: "ds-oura".to_owned(),
        category: EventCategory::Sleep,
        event_type: "NightSleep".to_owned(),
        source_name: "Oura".to_owned(),
        duration_seconds: Some(28800),
        start_datetime: start,
        end_datetime: start + Duration::hours(8),
        zone_offset: Some("-05:00".to_owned()),
    }
}

#[test]
fn test_event_record_workout_category() {
    let event = workout_event();
    assert_eq!(event.category, EventCategory::Workout);
    assert_eq!(event.event_type, "Run");
    assert_eq!(event.source_name, "Garmin");
    assert!(event.external_id.is_some());
}

#[test]
fn test_event_record_sleep_category() {
    let event = sleep_event();
    assert_eq!(event.category, EventCategory::Sleep);
    assert_eq!(event.event_type, "NightSleep");
    assert_eq!(event.source_name, "Oura");
    assert!(event.external_id.is_none());
}

#[test]
fn test_event_category_workout_serialization() {
    let json = serde_json::to_string(&EventCategory::Workout)
        .expect("EventCategory::Workout should serialize");
    assert_eq!(json, "\"Workout\"");
}

#[test]
fn test_event_category_sleep_serialization() {
    let json = serde_json::to_string(&EventCategory::Sleep)
        .expect("EventCategory::Sleep should serialize");
    assert_eq!(json, "\"Sleep\"");
}

#[test]
fn test_workout_details_all_fields() {
    let start = Utc::now();
    let workout = WorkoutDetails {
        event: EventRecord {
            id: "wkt-full".to_owned(),
            external_id: Some("ext-123".to_owned()),
            data_source_id: "ds-garmin".to_owned(),
            category: EventCategory::Workout,
            event_type: "Cycling".to_owned(),
            source_name: "Garmin".to_owned(),
            duration_seconds: Some(5400),
            start_datetime: start,
            end_datetime: start + Duration::seconds(5400),
            zone_offset: None,
        },
        heart_rate_min: Some(85),
        heart_rate_max: Some(190),
        heart_rate_avg: Some(155.0),
        energy_burned: Some(920.0),
        distance: Some(42_195.0),
        steps_count: Some(45_000),
        max_speed: Some(12.8),
        average_speed: Some(7.8),
        max_watts: Some(400),
        average_watts: Some(250),
        moving_time_seconds: Some(5200),
        total_elevation_gain: Some(750.0),
        elev_high: Some(1500.0),
        elev_low: Some(200.0),
    };

    assert_eq!(workout.heart_rate_min, Some(85));
    assert_eq!(workout.heart_rate_max, Some(190));
    assert_eq!(workout.heart_rate_avg, Some(155.0));
    assert_eq!(workout.energy_burned, Some(920.0));
    assert_eq!(workout.distance, Some(42_195.0));
    assert_eq!(workout.steps_count, Some(45_000));
    assert_eq!(workout.max_speed, Some(12.8));
    assert_eq!(workout.average_speed, Some(7.8));
    assert_eq!(workout.max_watts, Some(400));
    assert_eq!(workout.average_watts, Some(250));
    assert_eq!(workout.moving_time_seconds, Some(5200));
    assert_eq!(workout.total_elevation_gain, Some(750.0));
    assert_eq!(workout.elev_high, Some(1500.0));
    assert_eq!(workout.elev_low, Some(200.0));
}

#[test]
fn test_workout_details_minimal() {
    let start = Utc::now();
    let workout = WorkoutDetails {
        event: EventRecord {
            id: "wkt-min".to_owned(),
            external_id: None,
            data_source_id: "ds-apple".to_owned(),
            category: EventCategory::Workout,
            event_type: "Yoga".to_owned(),
            source_name: "Apple".to_owned(),
            duration_seconds: Some(1800),
            start_datetime: start,
            end_datetime: start + Duration::minutes(30),
            zone_offset: None,
        },
        heart_rate_min: None,
        heart_rate_max: None,
        heart_rate_avg: None,
        energy_burned: None,
        distance: None,
        steps_count: None,
        max_speed: None,
        average_speed: None,
        max_watts: None,
        average_watts: None,
        moving_time_seconds: None,
        total_elevation_gain: None,
        elev_high: None,
        elev_low: None,
    };

    assert!(workout.heart_rate_min.is_none());
    assert!(workout.heart_rate_max.is_none());
    assert!(workout.heart_rate_avg.is_none());
    assert!(workout.energy_burned.is_none());
    assert!(workout.distance.is_none());
    assert!(workout.steps_count.is_none());
    assert!(workout.max_speed.is_none());
    assert!(workout.average_speed.is_none());
    assert!(workout.max_watts.is_none());
    assert!(workout.average_watts.is_none());
    assert!(workout.moving_time_seconds.is_none());
    assert!(workout.total_elevation_gain.is_none());
    assert!(workout.elev_high.is_none());
    assert!(workout.elev_low.is_none());
}

#[test]
fn test_workout_heart_rate_ordering() {
    let start = Utc::now();
    let workout = WorkoutDetails {
        event: EventRecord {
            id: "wkt-hr".to_owned(),
            external_id: None,
            data_source_id: "ds-001".to_owned(),
            category: EventCategory::Workout,
            event_type: "Run".to_owned(),
            source_name: "Garmin".to_owned(),
            duration_seconds: Some(3600),
            start_datetime: start,
            end_datetime: start + Duration::hours(1),
            zone_offset: None,
        },
        heart_rate_min: Some(90),
        heart_rate_max: Some(185),
        heart_rate_avg: Some(155.0),
        energy_burned: None,
        distance: None,
        steps_count: None,
        max_speed: None,
        average_speed: None,
        max_watts: None,
        average_watts: None,
        moving_time_seconds: None,
        total_elevation_gain: None,
        elev_high: None,
        elev_low: None,
    };

    let hr_min = workout.heart_rate_min.expect("min HR set");
    let hr_max = workout.heart_rate_max.expect("max HR set");
    let hr_avg = workout.heart_rate_avg.expect("avg HR set");

    assert!(hr_min < hr_max, "min HR should be less than max HR");
    assert!(f64::from(hr_min) <= hr_avg, "min HR should be <= avg HR");
    assert!(hr_avg <= f64::from(hr_max), "avg HR should be <= max HR");
}

#[test]
fn test_workout_distance_and_speed_consistency() {
    let start = Utc::now();
    let workout = WorkoutDetails {
        event: EventRecord {
            id: "wkt-spd".to_owned(),
            external_id: None,
            data_source_id: "ds-001".to_owned(),
            category: EventCategory::Workout,
            event_type: "Cycling".to_owned(),
            source_name: "Garmin".to_owned(),
            duration_seconds: Some(3600),
            start_datetime: start,
            end_datetime: start + Duration::hours(1),
            zone_offset: None,
        },
        heart_rate_min: None,
        heart_rate_max: None,
        heart_rate_avg: None,
        energy_burned: None,
        distance: Some(36_000.0),
        steps_count: None,
        max_speed: Some(15.0),
        average_speed: Some(10.0),
        max_watts: None,
        average_watts: None,
        moving_time_seconds: Some(3600),
        total_elevation_gain: None,
        elev_high: None,
        elev_low: None,
    };

    let avg_speed = workout.average_speed.expect("avg speed set");
    let max_speed = workout.max_speed.expect("max speed set");
    assert!(
        avg_speed <= max_speed,
        "average speed should not exceed max speed"
    );
}

#[test]
fn test_workout_watts_and_elevation() {
    let start = Utc::now();
    let workout = WorkoutDetails {
        event: EventRecord {
            id: "wkt-pwr".to_owned(),
            external_id: None,
            data_source_id: "ds-001".to_owned(),
            category: EventCategory::Workout,
            event_type: "Cycling".to_owned(),
            source_name: "Garmin".to_owned(),
            duration_seconds: Some(7200),
            start_datetime: start,
            end_datetime: start + Duration::hours(2),
            zone_offset: None,
        },
        heart_rate_min: None,
        heart_rate_max: None,
        heart_rate_avg: None,
        energy_burned: None,
        distance: None,
        steps_count: None,
        max_speed: None,
        average_speed: None,
        max_watts: Some(450),
        average_watts: Some(280),
        moving_time_seconds: None,
        total_elevation_gain: Some(1200.0),
        elev_high: Some(2000.0),
        elev_low: Some(500.0),
    };

    assert!(
        workout.average_watts.expect("avg watts set")
            <= workout.max_watts.expect("max watts set"),
        "average watts should not exceed max watts"
    );
    assert!(
        workout.elev_low.expect("elev low set")
            <= workout.elev_high.expect("elev high set"),
        "lowest elevation should not exceed highest elevation"
    );
}

#[test]
fn test_workout_details_json_roundtrip() {
    let start = Utc::now();
    let workout = WorkoutDetails {
        event: EventRecord {
            id: "wkt-rt".to_owned(),
            external_id: Some("strava-555".to_owned()),
            data_source_id: "ds-001".to_owned(),
            category: EventCategory::Workout,
            event_type: "Run".to_owned(),
            source_name: "Strava".to_owned(),
            duration_seconds: Some(2700),
            start_datetime: start,
            end_datetime: start + Duration::minutes(45),
            zone_offset: Some("+01:00".to_owned()),
        },
        heart_rate_min: Some(100),
        heart_rate_max: Some(175),
        heart_rate_avg: Some(150.5),
        energy_burned: Some(450.0),
        distance: Some(10_000.0),
        steps_count: Some(12_000),
        max_speed: Some(5.0),
        average_speed: Some(3.7),
        max_watts: None,
        average_watts: None,
        moving_time_seconds: Some(2650),
        total_elevation_gain: Some(120.0),
        elev_high: Some(350.0),
        elev_low: Some(230.0),
    };

    let json = serde_json::to_string(&workout)
        .expect("WorkoutDetails should serialize to JSON");
    let roundtripped: WorkoutDetails = serde_json::from_str(&json)
        .expect("JSON should deserialize back to WorkoutDetails");

    assert_eq!(roundtripped.event.id, workout.event.id);
    assert_eq!(roundtripped.event.category, EventCategory::Workout);
    assert_eq!(roundtripped.heart_rate_avg, workout.heart_rate_avg);
    assert_eq!(roundtripped.distance, workout.distance);
    assert_eq!(roundtripped.total_elevation_gain, workout.total_elevation_gain);
}

#[test]
fn test_event_record_duration_seconds() {
    let start = Utc::now();
    let duration_secs = 5400_u32;
    let event = EventRecord {
        id: "evt-dur".to_owned(),
        external_id: None,
        data_source_id: "ds-001".to_owned(),
        category: EventCategory::Workout,
        event_type: "Swim".to_owned(),
        source_name: "Apple".to_owned(),
        duration_seconds: Some(duration_secs),
        start_datetime: start,
        end_datetime: start + Duration::seconds(i64::from(duration_secs)),
        zone_offset: None,
    };

    assert_eq!(event.duration_seconds, Some(5400));
    // Verify duration_seconds matches the time span between start and end
    let computed_duration = event
        .end_datetime
        .signed_duration_since(event.start_datetime)
        .num_seconds();
    assert_eq!(computed_duration, i64::from(duration_secs));
}
