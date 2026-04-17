<div align="center">

# 🏆 BioSwarm Engine v3.5

**Production-ready multi-source intelligence swarm for AI-powered research**

[![Rust](https://img.shields.io/badge/Rust-1.75+-orange.svg?style=flat-square&logo=rust)](https://www.rust-lang.org)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg?style=flat-square)](https://opensource.org/licenses/MIT)
[![Version](https://img.shields.io/badge/version-3.5.0-blue.svg?style=flat-square)](https://github.com/sayuru-akash/bioswarm-engine/releases)
[![Status](https://img.shields.io/badge/status-restored%20production%20baseline-success.svg?style=flat-square)](https://github.com/sayuru-akash/bioswarm-engine)

[Overview](#overview) • [Features](#features) • [Installation](#installation) • [Usage](#usage) • [Configuration](#configuration) • [API](#api) • [Roadmap](#roadmap)

</div>

---

## 📸 Preview

```bash
$ bioswarm run --query "latest AI technology trends 2026" --depth 3

🏆 BioSwarm v3.5 - Starting intelligence swarm...
✅ SQLite database initialized
✅ Search clients ready
🤖 Spawning specialist agents...
✅ Swarm execution complete!
📊 Execution summary ready
📁 Exports written: Markdown, JSON, HTML, CSV
```

---

## 📋 Overview

BioSwarm Engine is a **production-ready, multi-source intelligence CLI** built in Rust. It orchestrates specialist AI research flows to gather fresh intelligence, persist execution history, and export usable reports.

**Current strengths:**
- ⚡ **Production CLI flows** for run, resume, export, history, and status
- 🔄 **Resumable runs** with SQLite-backed checkpoints
- 📊 **Multi-format exports** in Markdown, JSON, HTML, and CSV
- 🎯 **Confidence scoring** and executive summary generation
- 🗄️ **Persistent history** with searchable run records and trend deltas
- 🧪 **Tested baseline** with integration and snapshot coverage

---

## ✨ Features

| Feature | Description | Status |
|---------|-------------|--------|
| 🤖 **Swarm Research Execution** | Multi-agent business intelligence orchestration | ✅ |
| 🗄️ **SQLite Persistence** | Schema migrations, checkpoint/resume, run history | ✅ |
| 📤 **Multi-Format Exports** | Markdown, JSON, HTML, CSV with report templates | ✅ |
| 🔄 **Resume Flow** | Recover and continue from stored checkpoints | ✅ |
| 📈 **Trend Analysis** | Delta summaries across multiple runs | ✅ |
| 🎯 **Confidence Scoring** | Per-run and aggregated confidence signals | ✅ |
| 🧹 **Deduplication** | Duplicate insight cleanup in output generation | ✅ |
| 🔍 **Fresh Research Support** | Fireworks generation with optional Exa freshness layer | ✅ |
| 🛡️ **Schema Migrations** | Automatic database upgrades on startup | ✅ |
| 🧪 **Testing** | Integration tests + snapshot tests | ✅ |
| 🌐 **REST API Server** | HTTP entrypoint for automation | ✅ |
| 🚀 **Provider-flexible model backends** | Fireworks, Ollama, OpenAI-compatible runtime switching | ✅ |

---

## 🚀 Installation

### Build from Source

```bash
git clone https://github.com/sayuru-akash/bioswarm-engine.git
cd bioswarm-engine
cargo build --release
cargo test
```

### Prerequisites

- **Rust** 1.75+ ([Install](https://rustup.rs))
- **API keys:**
  - [Fireworks AI](https://fireworks.ai) for text generation
  - [Exa AI](https://exa.ai) optional, for fresher research/search enrichment

---

## 🎮 Usage

### Basic Run

```bash
export FIREWORKS_API_KEY="your_key_here"
bioswarm run --query "AI market trends 2026"
```

### Advanced Usage

```bash
bioswarm run \
  --query "sri lanka fintech opportunities" \
  --depth 3 \
  --formats markdown,json,html,csv \
  --output-dir ./reports \
  --database-path ./bioswarm.db

bioswarm history --limit 20
bioswarm status
bioswarm export --execution-id "<id>" --formats markdown,json
bioswarm resume --execution-id "<id>"
```

### Backend-flexible runtime examples

```bash
# Fireworks
export FIREWORKS_API_KEY="fw_..."
bioswarm run --query "AI market trends" --backend fireworks --model accounts/fireworks/models/kimi-k2-instruct

# Ollama
export OLLAMA_API_KEY="local-or-placeholder"
bioswarm run --query "AI market trends" \
  --backend ollama \
  --model kimi-k2.5:cloud \
  --api-base-url http://127.0.0.1:11434/v1 \
  --api-key-env OLLAMA_API_KEY

# OpenAI-compatible
export OPENAI_API_KEY="sk-..."
bioswarm run --query "AI market trends" \
  --backend openai-compatible \
  --model gpt-4.1-mini \
  --api-base-url https://api.openai.com/v1 \
  --api-key-env OPENAI_API_KEY
```

### CLI Commands

| Command | Description |
|---------|-------------|
| `run` | Execute a new swarm |
| `resume` | Resume from a saved checkpoint |
| `export` | Export an existing run |
| `history` | List past runs |
| `status` | Show current status and checkpoints |

---

## ⚙️ Configuration

### Environment Variables

```bash
# Required
export FIREWORKS_API_KEY="fw_..."

# Optional
export EXA_API_KEY="exa_..."
export DATABASE_PATH="./bioswarm.db"
export OUTPUT_DIR="./outputs"
export RATE_LIMIT_RPM=60
export BIOSWARM_DEPTH=2
export RUST_LOG=info
```

### Config File (`bioswarm.toml`)

```toml
fireworks_api_key = "fw_..."
exa_api_key = "exa_..."
rate_limit_rpm = 60
database_path = "bioswarm.db"
output_dir = "outputs"
depth = 2
formats = ["markdown", "json", "html", "csv"]
backend = "fireworks"
model = "accounts/fireworks/models/kimi-k2-instruct"
api_base_url = "https://api.fireworks.ai/inference/v1"
api_key_env = "FIREWORKS_API_KEY"
```

---

## 🌐 API

BioSwarm includes an HTTP API server for automation:

```bash
./target/release/bioswarm-server
```

### Endpoints

| Endpoint | Method | Description |
|----------|--------|-------------|
| `/health` | GET | Health check |
| `/api/v1/swarm/run` | POST | Execute swarm |
| `/api/v1/swarm/status` | GET | Get current status/checkpoints |

---

## 🏗️ Architecture

```text
CLI / API
   ↓
Orchestrator
   ↓
Agent execution + checkpointing
   ↓
Search + generation clients
   ↓
SQLite persistence + export pipeline
```

Core files:
- `src/main.rs` CLI entry
- `src/server.rs` API server
- `src/database.rs` persistence + migrations
- `src/orchestrator.rs` swarm execution + checkpointing
- `src/exports.rs` export writers
- `tests/` integration and snapshot coverage

---

## 🛣️ Roadmap

Planned next improvements:
- richer provider-specific capability negotiation
- stronger retry/backoff behavior for live provider calls
- richer report presentation and executive dashboards
- stronger CI/release polish
- deeper structured extraction from model outputs

Important: roadmap items above are next-step improvements beyond the currently shipped backend-flexible runtime support.

---

## 📜 License

MIT
