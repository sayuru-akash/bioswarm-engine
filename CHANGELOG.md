# Changelog

All notable changes to BioSwarm Engine will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [2.0.0] - 2026-04-07

### Added - Complete Rewrite 🎉

#### Multi-Source Intelligence
- **Exa Web Search API** integration for real-time web data
- **Fireworks AI** integration (Kimi K2.5 Turbo only)
- Cross-validation from multiple sources
- No caching - fresh data every run

#### Core Engine
- **14 parallel agents** with specialized functions
- **Tokio async runtime** for parallel execution
- **Rate limiting** (60 RPM token bucket)
- **Retry logic** (3 attempts with exponential backoff)
- Full Rust type safety

#### New Architecture
- Modular design (models, search, orchestrator, agents, config)
- Clean separation of concerns
- Optimized dependencies (8 crates vs 25 in v1.0)
- 2.1MB release binary (vs 3.5MB in v1.0)

#### Production Ready
- Complete README with usage instructions
- MIT License
- .env.sample for configuration
- .gitignore for security
- Cargo.lock for reproducible builds

#### API Server
- Actix-web REST API
- Health check endpoint
- Agent listing endpoint
- Swagger-ready structure

## [1.0.0] - 2026-04-07

### Added - Initial Release
- Basic 14-agent swarm architecture
- Fireworks API integration
- CLI interface
- Report generation

### Archived
- Superseded by v2.0.0 with multi-source intelligence
