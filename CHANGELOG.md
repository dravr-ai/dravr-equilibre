# Changelog

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
