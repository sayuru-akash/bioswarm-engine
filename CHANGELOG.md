# Changelog

## 3.5.0 - 2026-04-18
- restored the correct BioSwarm production code line from the real v3 baseline
- removed the accidental nested-folder regression and version drift introduced after v3
- re-established the proper root repo structure, tests, exports, and persistence-oriented architecture
- validated `cargo build`, `cargo test`, and `cargo clippy -- -D warnings` on the corrected tree
- promoted this corrected state as the forward-working production baseline

## 3.0.0
- rebuilt the project into a production-oriented CLI with subcommands
- added runtime config loading from env and optional bioswarm.toml
- added SQLite migrations, run history, checkpoint recovery, and export rehydration
- added report confidence scoring, deduplication, trend deltas, and action extraction
- added integration tests plus snapshot tests for exports
- aligned branch/repo metadata and docs for the production release
