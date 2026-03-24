// ABOUTME: Structured error types for the health and wellness domain models
// ABOUTME: Defines EquilibreError enum with domain-specific variants and Result alias
//
// SPDX-License-Identifier: Apache-2.0
// Copyright (c) 2026 dravr.ai

//! Error types for the dravr-equilibre health domain library.
//!
//! All errors use structured variants rather than string-based ad-hoc errors.
//! The [`EquilibreError`] enum covers domain-specific failure modes including
//! missing fields, out-of-range values, and provider capability errors.

use thiserror::Error;

/// Result type alias for equilibre operations.
pub type EquilibreResult<T> = Result<T, EquilibreError>;

/// Structured error type for the health domain library.
///
/// Each variant represents a distinct failure mode with typed fields
/// for programmatic error handling.
#[derive(Debug, Error)]
pub enum EquilibreError {
    /// A required field is missing from the input.
    #[error("missing required field: '{field}'")]
    MissingField {
        /// Name of the missing field.
        field: String,
    },

    /// A value is outside the valid domain range.
    #[error("value out of range for '{field}': {value} not in [{min}, {max}]")]
    OutOfRange {
        /// Name of the field or parameter.
        field: String,
        /// The out-of-range value.
        value: f64,
        /// Minimum allowed value.
        min: f64,
        /// Maximum allowed value.
        max: f64,
    },

    /// Sleep stages serialization or deserialization failed.
    #[error("stages serialization error: {reason}")]
    StagesSerialization {
        /// Description of the serialization problem.
        reason: String,
    },

    /// The start time is not before the end time.
    #[error("invalid time range: start must be before end")]
    InvalidTimeRange,

    /// A provider does not support the requested capability.
    #[error("provider '{provider}' does not support capability '{capability}'")]
    ProviderNotSupported {
        /// Name of the provider.
        provider: String,
        /// Name of the unsupported capability.
        capability: String,
    },

    /// A data source with the given ID already exists.
    #[error("duplicate data source: '{source_id}'")]
    DuplicateDataSource {
        /// ID of the duplicate data source.
        source_id: String,
    },

    /// Serialization or deserialization failed.
    #[error("serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
}

impl EquilibreError {
    /// Create a missing field error.
    pub fn missing_field(field: impl Into<String>) -> Self {
        Self::MissingField {
            field: field.into(),
        }
    }

    /// Create an out-of-range error.
    pub fn out_of_range(field: impl Into<String>, value: f64, min: f64, max: f64) -> Self {
        Self::OutOfRange {
            field: field.into(),
            value,
            min,
            max,
        }
    }

    /// Create a stages serialization error.
    pub fn stages_serialization(reason: impl Into<String>) -> Self {
        Self::StagesSerialization {
            reason: reason.into(),
        }
    }

    /// Create a provider not supported error.
    pub fn provider_not_supported(
        provider: impl Into<String>,
        capability: impl Into<String>,
    ) -> Self {
        Self::ProviderNotSupported {
            provider: provider.into(),
            capability: capability.into(),
        }
    }

    /// Create a duplicate data source error.
    pub fn duplicate_data_source(source_id: impl Into<String>) -> Self {
        Self::DuplicateDataSource {
            source_id: source_id.into(),
        }
    }
}
