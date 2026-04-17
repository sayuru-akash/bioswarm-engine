# 🏆 BioSwarm Engine v3.5

Production-ready multi-source intelligence CLI for research swarms, resumable runs, SQLite-backed history, and multi-format exports.

[![Rust](https://img.shields.io/badge/Rust-1.75+-orange.svg)](https://www.rust-lang.org)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Version](https://img.shields.io/badge/version-3.5.0-blue.svg)]()

## What is included

- reliable CLI with `run`, `resume`, `export`, `history`, `status`
- resumable checkpoint flow backed by SQLite
- schema migration bootstrap on startup
- configurable depth, agents, output dir, database path, and export formats
- export outputs in Markdown, JSON, HTML, and CSV
- report deduplication, confidence scoring, executive summaries, and action extraction
- searchable run history and trend deltas
- REST server entrypoint for automation
- integration tests and output snapshot tests
- cleaned release baseline restored from the real v3 production line

## What is new in v3.5

- restored the correct v3 production code line after an accidental nested-folder regression
- re-established the proper root repo layout
- removed the temporary versioning drift to `v1.1`
- kept the real production architecture centered on SQLite, exports, history, and resume flows
- validated build, tests, and strict clippy on the corrected tree
- set this as the forward-working production baseline

## Quick start

```bash
cp .env.sample .env
cargo build --release
cargo test
./target/release/bioswarm run --query "ai market intelligence"
```

## CLI examples

```bash
bioswarm run --query "sri lanka fintech opportunities" --depth 3 --formats markdown,json,html,csv
bioswarm history --limit 20
bioswarm status
bioswarm export --execution-id <id> --formats markdown,json
bioswarm resume --execution-id <id>
```

## Config

Environment variables or `bioswarm.toml`:

```toml
fireworks_api_key = "..."
exa_api_key = "..."
rate_limit_rpm = 60
database_path = "bioswarm.db"
output_dir = "outputs"
depth = 2
formats = ["markdown", "json", "html", "csv"]
```

Required:
- `FIREWORKS_API_KEY`

Optional:
- `EXA_API_KEY`
- `DATABASE_PATH`
- `OUTPUT_DIR`
- `RATE_LIMIT_RPM`
- `BIOSWARM_DEPTH`

## Reliability work included

- schema migrations table added
- checkpoint saving after each completed agent
- resume flow supported from last checkpoint
- integration tests for status and run/export path
- snapshot tests for markdown, csv, and html outputs
- stronger config validation with fail-fast errors

## Packaging and release readiness

- release build supported with `cargo build --release`
- API server binary included as `bioswarm-server`
- suitable for GitHub Actions and tagged releases

## Repo structure

- `src/main.rs` CLI entry
- `src/server.rs` API server
- `src/database.rs` persistence + migrations
- `src/orchestrator.rs` swarm execution + checkpointing
- `src/exports.rs` export writers
- `tests/` integration and snapshot tests

## Attribution

Main author commits are configured for Sayuru Akash. Co-author credit can be added for Sayuru's OpenClaw Bot ❤️ in commit trailers when desired.
