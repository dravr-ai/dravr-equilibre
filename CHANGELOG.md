# Changelog

## [0.2.5] — 2026-09-04

### Fixed

- fix(deps): bump quinn-proto past the memory-exhaustion advisory
- fix: repair the SessionStart bootstrap guard for an empty .build

### Other

- Delete the three provider handler traits with no implementors
- chore(deps): bump dravr-tronc 0.5.3 -> 0.6.2
- chore(register): ledger + weekly phase review
- chore(register): point at dravr-carnet, the dravr-family register



## [0.2.4] — 2026-06-19

### Changed

- deps: migrate `dravr-equilibre-mcp` and `dravr-equilibre-server` to dravr-tronc
  0.5.3 (dual-era MCP engine); state is `Arc<S>` directly (tronc no longer wraps
  it in a `RwLock`). The core `dravr-equilibre` crate is unchanged.

## [0.2.3] — 2026-04-10

### Other

- build: reduce tokio feature footprint to minimal set



## [0.2.2] — 2026-03-31



## [0.2.1] — 2026-03-26

### Fixed

- fix: scope release version bump to [package] section to protect external deps

### Other

- deps: bump dravr-tronc to 0.2 with error notification support
- style: fix formatting in test files



## [0.2.0] — 2026-03-25



## [0.1.0] — 2026-03-25

### Added

- feat: initial scaffold — composition-based health domain models 3 workspace crates (core, MCP, server), provider traits, device tracking, 36 tests



## [0.1.0] - 2026-03-24

### Added
- Composition-based provider traits (OAuthHandler, WorkoutHandler, ContinuousDataHandler)
- ProviderStrategy for composing provider capabilities
- DataSource model with device type and software version tracking
- DevicePriority and ProviderPriority for deduplication
- EventRecord base model with WorkoutDetails and SleepDetails
- StoredSleepSession with is_nap flag and JSONB stages
- StoredRecoveryMetrics for daily recovery/readiness
- StoredHealthMetrics for body composition and vitals
- SyncStatus/SyncResult for data synchronization tracking
- MCP server via dravr-tronc
- REST API + MCP unified server
