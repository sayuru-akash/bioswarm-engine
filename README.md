# 🏆 BioSwarm Engine v2.0

**Multi-Source Intelligence Swarm with Real-Time Web Search**

[![Rust](https://img.shields.io/badge/Rust-1.75+-orange.svg)](https://www.rust-lang.org)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Build](https://img.shields.io/badge/build-passing-brightgreen.svg)]()

> **14 parallel agents** gathering fresh intelligence using **Exa Web Search** + **Fireworks AI** (Kimi K2.5 Turbo)

---

## 🚀 Quick Start

```bash
# 1. Clone and enter directory
cd bioswarm-v2

# 2. Set up environment (copy and fill in your keys)
cp .env.sample .env
# Edit .env with your actual API keys

# 3. Build release binary
cargo build --release

# 4. Run the swarm
./target/release/bioswarm-v2

# 5. Check output
cat /tmp/bioswarm_v2_report.md
```

---

## 📋 Prerequisites

- **Rust** 1.75+ ([Install](https://rustup.rs))
- **API Keys:**
  - [Fireworks AI](https://fireworks.ai) - For text generation (Kimi K2.5 Turbo)
  - [Exa AI](https://exa.ai) - For web search (optional, enhances freshness)

---

## 🎯 What It Does

BioSwarm v2.0 orchestrates **14 specialist agents** in parallel:

| Agent | Function | Data Source |
|-------|----------|-------------|
| DeepResearcher | Market trends & AI adoption | Exa + Fireworks |
| GapAnalyzer | Market void identification | Exa + Fireworks |
| OpportunityScorer | ROI-ranked opportunities | Fireworks |
| CompetitorTracker | Rival intelligence | Exa + Fireworks |
| InnovationScout | Emerging tech tracking | Exa + Fireworks |
| StrategyFormulator | Action roadmaps | Fireworks |
| QualityValidator | Cross-check validation | Fireworks |
| DeploymentTester | Feasibility assessment | Fireworks |
| SentimentAnalyzer | Market mood analysis | Exa + Fireworks |
| PricingIntelligence | Rate benchmarking | Exa + Fireworks |
| TalentScout | Hiring intelligence | Exa + Fireworks |
| FundingTracker | Investment tracking | Exa + Fireworks |
| RegulatoryWatcher | Compliance monitoring | Exa + Fireworks |
| ClientIntelligence | Account signal analysis | Fireworks |

---

## 🏗️ Architecture

```
┌──────────────────────────────────────────────────────┐
│              BioSwarm Engine v2.0                    │
├──────────────────────────────────────────────────────┤
│  CLI Layer (main.rs)  →  API Layer (server.rs)       │
├──────────────────────────────────────────────────────┤
│           Orchestrator (orchestrator.rs)             │
│              Parallel Agent Spawning                 │
├──────────────────────────────────────────────────────┤
│  ┌─────────┐ ┌─────────┐ ┌─────────┐ ┌─────────┐    │
│  │ Agent 1 │ │ Agent 2 │ │   ...   │ │ Agent 14│    │
│  └─────────┘ └─────────┘ └─────────┘ └─────────┘    │
├──────────────────────────────────────────────────────┤
│         Exa API          │      Fireworks API        │
│     (Web Search)         │   (Kimi K2.5 Turbo)       │
└──────────────────────────────────────────────────────┘
```

---

## 📁 Project Structure

```
bioswarm-v2/
├── Cargo.toml              # Dependencies & metadata
├── Cargo.lock              # Locked dependency versions
├── README.md               # This file
├── LICENSE                 # MIT License
├── .gitignore             # Git ignore rules
├── .env.sample            # Environment template (NO REAL KEYS)
├── src/
│   ├── main.rs            # CLI entry point
│   ├── server.rs          # API server entry (optional)
│   ├── lib.rs             # Core engine
│   ├── models.rs          # Data structures
│   ├── search.rs          # API clients (Exa + Fireworks)
│   ├── orchestrator.rs    # Parallel execution
│   ├── agents.rs          # Agent implementations
│   └── config.rs          # Configuration
├── tests/                 # Integration tests
└── docs/                  # Documentation
```

---

## 🔧 Configuration

Create `.env` file (copy from `.env.sample`):

```bash
# Required
FIREWORKS_API_KEY=your_fireworks_api_key_here

# Optional (enhances search freshness)
EXA_API_KEY=your_exa_api_key_here

# Optional settings
RUST_LOG=info
RATE_LIMIT_RPM=60
```

**⚠️ Never commit `.env` with real keys!**

---

## 🛠️ Development

```bash
# Development build
cargo build

# Run with logging
RUST_LOG=debug cargo run

# Run tests
cargo test

# Format code
cargo fmt

# Run lints
cargo clippy -- -D warnings

# Release build (optimized)
cargo build --release
```

---

## 📊 Performance

| Metric | Value |
|--------|-------|
| **Agents** | 14 parallel |
| **Execution Time** | ~36-50 seconds |
| **Binary Size** | ~2.1MB (release) |
| **Memory Usage** | ~50MB |
| **Build Time** | ~60-70 seconds |
| **Rate Limit** | 60 RPM (configurable) |

---

## 🔒 Security

- **No hardcoded keys** - All API keys via environment
- **No sensitive data in repo** - `.env` is gitignored
- **Rate limiting** - Built-in token bucket
- **Non-root Docker** - Production deployment ready

---

## 📝 Output

Generates comprehensive markdown report:
- Market trends & analysis
- Competitive intelligence
- Emerging technologies
- Talent market data
- Funding landscape
- Regulatory updates
- Strategic recommendations

Output saved to: `/tmp/bioswarm_v2_report.md`

---

## 🤝 Contributing

1. Fork the repository
2. Create feature branch: `git checkout -b feature/amazing`
3. Commit changes: `git commit -m 'Add amazing feature'`
4. Push to branch: `git push origin feature/amazing`
5. Open Pull Request

---

## 📜 License

MIT License - see [LICENSE](LICENSE) file

---

## 🙏 Acknowledgments

- [Fireworks AI](https://fireworks.ai) - Kimi K2.5 Turbo API
- [Exa AI](https://exa.ai) - Web search API
- [Tokio](https://tokio.rs) - Async runtime
- [Rust](https://www.rust-lang.org) - Programming language

---

**Built with 💛 by Sayuru's OpenClaw Bot ❤️**

*Production-ready multi-source intelligence engine*