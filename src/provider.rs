// ABOUTME: Composition-based provider traits for health data synchronization
// ABOUTME: OAuthHandler, WorkoutHandler, ContinuousDataHandler traits and ProviderStrategy
//
// SPDX-License-Identifier: Apache-2.0
// Copyright (c) 2026 dravr.ai

//! Composition-based provider traits for health data synchronization.
//!
//! Providers compose only the handlers they support:
//! - Strava: `OAuthHandler` + `WorkoutHandler`
//! - Garmin: `OAuthHandler` + `WorkoutHandler` + `ContinuousDataHandler`
//! - Oura: `OAuthHandler` + `ContinuousDataHandler`
//!
//! The [`ProviderStrategy`] struct holds optional boxed trait objects
//! for each handler, enabling runtime composition.

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::error::EquilibreResult;
use crate::recovery::StoredRecoveryMetrics;
use crate::sleep::StoredSleepSession;
use crate::workout::WorkoutDetails;

/// Credentials returned from OAuth operations.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Credentials {
    /// OAuth access token.
    pub access_token: String,
    /// OAuth refresh token (not all providers issue one).
    pub refresh_token: Option<String>,
    /// Token expiration time.
    pub expires_at: Option<DateTime<Utc>>,
    /// Granted OAuth scopes.
    pub scopes: Vec<String>,
}

/// A batch of continuous metric data points from a provider.
///
/// Each batch represents a time series for a specific metric type
/// (e.g., heart rate, steps, respiratory rate).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContinuousMetricBatch {
    /// Identifier for the metric series type.
    pub series_type_id: u32,
    /// Time-value pairs for the metric.
    pub points: Vec<(DateTime<Utc>, f64)>,
}

/// Token lifecycle management for OAuth-based providers.
///
/// Handles the full OAuth flow: initial authorization, token refresh,
/// and token revocation.
#[async_trait]
pub trait OAuthHandler: Send + Sync {
    /// Exchange an authorization code for credentials.
    async fn authorize(&self, code: &str) -> EquilibreResult<Credentials>;

    /// Refresh an expired access token using the refresh token.
    async fn refresh_token(&self, refresh_token: &str) -> EquilibreResult<Credentials>;

    /// Revoke an access token, invalidating it on the provider side.
    async fn revoke(&self, access_token: &str) -> EquilibreResult<()>;
}

/// Workout and activity synchronization.
///
/// Fetches workout data from a provider for a given time range.
#[async_trait]
pub trait WorkoutHandler: Send + Sync {
    /// Fetch workouts for a user within the given time range.
    async fn fetch_workouts(
        &self,
        user_id: &str,
        start: DateTime<Utc>,
        end: DateTime<Utc>,
    ) -> EquilibreResult<Vec<WorkoutDetails>>;
}

/// 24/7 continuous monitoring data (sleep, recovery, HR, steps, etc.).
///
/// Fetches continuous health data streams from providers that
/// support background monitoring.
#[async_trait]
pub trait ContinuousDataHandler: Send + Sync {
    /// Fetch sleep sessions for a user within the given time range.
    async fn fetch_sleep(
        &self,
        user_id: &str,
        start: DateTime<Utc>,
        end: DateTime<Utc>,
    ) -> EquilibreResult<Vec<StoredSleepSession>>;

    /// Fetch recovery metrics for a user within the given time range.
    async fn fetch_recovery(
        &self,
        user_id: &str,
        start: DateTime<Utc>,
        end: DateTime<Utc>,
    ) -> EquilibreResult<Vec<StoredRecoveryMetrics>>;

    /// Fetch continuous metric batches (HR, steps, etc.) for a user within the given time range.
    async fn fetch_continuous_metrics(
        &self,
        user_id: &str,
        start: DateTime<Utc>,
        end: DateTime<Utc>,
    ) -> EquilibreResult<Vec<ContinuousMetricBatch>>;
}

/// A provider strategy composed of optional handlers.
///
/// Each provider composes only the handlers it supports. For example,
/// Strava provides OAuth + workouts but not continuous data, while
/// Garmin provides all three.
pub struct ProviderStrategy {
    /// Human-readable provider name.
    pub name: String,
    /// OAuth token lifecycle handler (most providers).
    pub oauth: Option<Box<dyn OAuthHandler>>,
    /// Workout synchronization handler.
    pub workouts: Option<Box<dyn WorkoutHandler>>,
    /// Continuous monitoring data handler.
    pub continuous_data: Option<Box<dyn ContinuousDataHandler>>,
}

impl ProviderStrategy {
    /// Create a new provider strategy with no handlers.
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            oauth: None,
            workouts: None,
            continuous_data: None,
        }
    }

    /// Set the OAuth handler.
    pub fn with_oauth(mut self, handler: Box<dyn OAuthHandler>) -> Self {
        self.oauth = Some(handler);
        self
    }

    /// Set the workout handler.
    pub fn with_workouts(mut self, handler: Box<dyn WorkoutHandler>) -> Self {
        self.workouts = Some(handler);
        self
    }

    /// Set the continuous data handler.
    pub fn with_continuous_data(mut self, handler: Box<dyn ContinuousDataHandler>) -> Self {
        self.continuous_data = Some(handler);
        self
    }

    /// Check whether this provider supports OAuth.
    pub fn supports_oauth(&self) -> bool {
        self.oauth.is_some()
    }

    /// Check whether this provider supports workout sync.
    pub fn supports_workouts(&self) -> bool {
        self.workouts.is_some()
    }

    /// Check whether this provider supports continuous data.
    pub fn supports_continuous_data(&self) -> bool {
        self.continuous_data.is_some()
    }
}
