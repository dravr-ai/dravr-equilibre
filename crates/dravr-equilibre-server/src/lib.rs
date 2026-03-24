// ABOUTME: Unified REST API + MCP server library for dravr-equilibre
// ABOUTME: Re-exports router, health, and auth modules
//
// SPDX-License-Identifier: Apache-2.0
// Copyright (c) 2026 dravr.ai

//! # dravr-equilibre-server
//!
//! Unified server exposing health domain models via REST API and MCP.
//! Combines the core library's models with HTTP endpoints and MCP protocol support.

/// Authentication middleware for bearer token validation
pub mod auth;
/// Health check endpoint
pub mod health;
/// Axum router combining REST and MCP routes
pub mod router;
