# Equilibre -- Health and Wellness Domain Models

[![CI](https://github.com/dravr-ai/dravr-equilibre/actions/workflows/ci.yml/badge.svg)](https://github.com/dravr-ai/dravr-equilibre/actions/workflows/ci.yml)
[![License](https://img.shields.io/badge/license-Apache--2.0-blue.svg)](LICENSE)

Health and wellness domain models for the Dravr platform. Composition-based provider traits, device tracking, data source management, and event persistence. Pure domain models with zero database or HTTP dependencies in the core crate.

## Table of Contents

- [Quick Start](#quick-start)
- [Domain Models](#domain-models)
- [Provider Traits](#provider-traits)
- [Deduplication](#deduplication)
- [REST API Server](#rest-api-server)
- [MCP Server](#mcp-server)
- [Architecture](#architecture)
- [License](#license)

## Quick Start

### Library (Rust)

```toml
[dependencies]
dravr-equilibre = "0.1"
```

```rust
use dravr_equilibre::data_source::{DataSource, DeviceType};
use dravr_equilibre::priority::{DevicePriority, ProviderPriority, resolve_duplicate};

// Device priority: Watch > Band > Ring > Phone > Scale > Unknown
let watch_priority = DevicePriority::priority(&DeviceType::Watch);
assert_eq!(watch_priority, 1);

// Provider priority: apple > garmin > polar > suunto > whoop > oura > fitbit > coros
let garmin_rank = ProviderPriority::priority("garmin");
assert_eq!(garmin_rank, 2);
```

### REST API Server

```bash
cargo run --bin dravr-equilibre-server -- serve --port 3200
```

```bash
curl http://localhost:3200/health
# {"status":"ok","service":"dravr-equilibre","version":"0.1.0"}
```

### MCP Server (stdio)

```bash
cargo run --bin dravr-equilibre-mcp -- --transport stdio
```

Or over HTTP:

```bash
cargo run --bin dravr-equilibre-mcp -- --transport http --port 3200
```

## Domain Models

| Model | Description |
|-------|-------------|
| `DataSource` | Device/provider tracking with model, type, and software version |
| `DeviceType` | Watch, Band, Phone, Ring, Scale, Unknown |
| `EventRecord` | Base event model (polymorphic: workout, sleep) |
| `WorkoutDetails` | Exercise metrics: HR, power, distance, elevation |
| `StoredSleepSession` | Sleep data with is_nap flag and JSONB stages |
| `SleepDetails` | Sleep event extending EventRecord |
| `StoredRecoveryMetrics` | Daily recovery/readiness (HRV, stress, body battery) |
| `StoredHealthMetrics` | Body composition and vitals (weight, BP, glucose) |
| `SyncStatus` / `SyncResult` | Sync lifecycle tracking |

## Provider Traits

Composition-based provider traits allow each provider to implement only what it supports:

```rust
use dravr_equilibre::provider::ProviderStrategy;

// Strava: OAuth + Workouts (no continuous data)
let strava = ProviderStrategy::new("strava");

// Garmin: OAuth + Workouts + Continuous Data
let garmin = ProviderStrategy::new("garmin");

// Oura: OAuth + Continuous Data (no workouts)
let oura = ProviderStrategy::new("oura");
```

| Trait | Purpose |
|-------|---------|
| `OAuthHandler` | Token lifecycle (authorize, refresh, revoke) |
| `WorkoutHandler` | Activity/workout sync |
| `ContinuousDataHandler` | 24/7 monitoring (sleep, recovery, HR, steps) |

## Deduplication

When multiple sources report the same data, priority determines the authoritative source:

**Device Priority** (lower = better): Watch (1) > Band (2) > Ring (3) > Phone (4) > Scale (5) > Unknown (99)

**Provider Priority** (lower = better): Apple (1) > Garmin (2) > Polar (3) > Suunto (4) > Whoop (5) > Oura (6) > Fitbit (7) > Coros (8)

Device priority is checked first; provider priority breaks ties.

## Architecture

```
dravr-equilibre/              # Core library (domain models + traits)
  src/
    lib.rs                    # Re-exports
    error.rs                  # EquilibreError
    data_source.rs            # DataSource, DeviceType
    priority.rs               # DevicePriority, ProviderPriority
    provider.rs               # OAuthHandler, WorkoutHandler, ContinuousDataHandler
    event.rs                  # EventRecord, EventCategory
    workout.rs                # WorkoutDetails
    sleep.rs                  # StoredSleepSession, SleepDetails, SleepStage
    recovery.rs               # StoredRecoveryMetrics
    health.rs                 # StoredHealthMetrics
    sync.rs                   # SyncStatus, SyncResult
  crates/
    dravr-equilibre-mcp/      # MCP server (lib + bin)
    dravr-equilibre-server/   # REST API + MCP unified server
```

## License

Licensed under Apache-2.0. See [LICENSE](LICENSE) for details.
