// ABOUTME: Sync lifecycle tracking models for data synchronization
// ABOUTME: SyncStatus enum and SyncResult for tracking provider sync operations
//
// SPDX-License-Identifier: Apache-2.0
// Copyright (c) 2026 dravr.ai

//! Sync lifecycle tracking for provider data synchronization.
//!
//! [`SyncStatus`] represents the state of a sync operation, while
//! [`SyncResult`] captures the outcome including counts and errors.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Status of a data synchronization operation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SyncStatus {
    /// Sync operation is queued and waiting to start.
    Pending,
    /// Sync operation is currently in progress.
    InProgress,
    /// Sync completed successfully.
    Completed,
    /// Sync failed with an error.
    Failed,
    /// Sync was cancelled before completion.
    Cancelled,
}

impl std::fmt::Display for SyncStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Pending => write!(f, "Pending"),
            Self::InProgress => write!(f, "InProgress"),
            Self::Completed => write!(f, "Completed"),
            Self::Failed => write!(f, "Failed"),
            Self::Cancelled => write!(f, "Cancelled"),
        }
    }
}

/// Result of a completed sync operation.
///
/// Captures the outcome metrics including how many records were
/// created, updated, skipped, or errored.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncResult {
    /// Final status of the sync operation.
    pub status: SyncStatus,
    /// Provider that was synced.
    pub provider: String,
    /// User whose data was synced.
    pub user_id: String,
    /// Number of records created.
    pub records_created: u32,
    /// Number of records updated.
    pub records_updated: u32,
    /// Number of records skipped (duplicates or unchanged).
    pub records_skipped: u32,
    /// Number of records that failed to process.
    pub records_errored: u32,
    /// Error message if the sync failed.
    pub error_message: Option<String>,
    /// When the sync operation started.
    pub started_at: DateTime<Utc>,
    /// When the sync operation completed.
    pub completed_at: Option<DateTime<Utc>>,
}
