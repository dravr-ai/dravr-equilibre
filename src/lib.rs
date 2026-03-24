// ABOUTME: Health and wellness domain models for the Dravr platform
// ABOUTME: Composition-based provider traits, device tracking, and data source management
//
// SPDX-License-Identifier: Apache-2.0
// Copyright (c) 2026 dravr.ai

#![deny(unsafe_code)]

//! # dravr-equilibre
//!
//! Health and wellness domain models providing composition-based provider traits,
//! device tracking, data source management, and event persistence for the Dravr platform.
//!
//! ## Features
//!
//! - **Provider traits** — `OAuthHandler`, `WorkoutHandler`, `ContinuousDataHandler`
//! - **Data sources** — device/provider tracking with type classification
//! - **Deduplication** — device and provider priority for resolving duplicates
//! - **Events** — polymorphic event model (workout, sleep)
//! - **Sleep** — sleep session tracking with stage-level detail
//! - **Recovery** — daily recovery/readiness metrics
//! - **Health** — body composition and vital signs
//! - **Sync** — sync lifecycle tracking
//!
//! ## Quick Start
//!
//! ```rust
//! use dravr_equilibre::data_source::{DataSource, DeviceType};
//! use dravr_equilibre::priority::{DevicePriority, ProviderPriority};
//!
//! let priority = DevicePriority::priority(&DeviceType::Watch);
//! assert_eq!(priority, 1);
//!
//! let provider_rank = ProviderPriority::priority("garmin");
//! assert_eq!(provider_rank, 2);
//! ```

// ============================================================================
// Domain modules
// ============================================================================

/// Data source and device tracking models
pub mod data_source;
/// Structured error types for the health domain
pub mod error;
/// Base event record model
pub mod event;
/// Body composition and vital sign metrics
pub mod health;
/// Deduplication priority system
pub mod priority;
/// Composition-based provider traits
pub mod provider;
/// Recovery and readiness metrics
pub mod recovery;
/// Sleep session models
pub mod sleep;
/// Sync lifecycle tracking
pub mod sync;
/// Workout details model
pub mod workout;

// ============================================================================
// Re-exports
// ============================================================================

pub use data_source::{DataSource, DeviceType};
pub use error::{EquilibreError, EquilibreResult};
pub use event::{EventCategory, EventRecord};
pub use health::StoredHealthMetrics;
pub use priority::{resolve_duplicate, DevicePriority, ProviderPriority};
pub use provider::{
    ContinuousDataHandler, ContinuousMetricBatch, Credentials, OAuthHandler, ProviderStrategy,
    WorkoutHandler,
};
pub use recovery::StoredRecoveryMetrics;
pub use sleep::{SleepDetails, SleepStage, SleepStageType, StoredSleepSession};
pub use sync::{SyncResult, SyncStatus};
pub use workout::WorkoutDetails;
