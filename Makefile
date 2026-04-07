# BioSwarm Engine Makefile

.PHONY: all build release test clean install docker run

# Default target
all: build

# Build development version
build:
	cargo build

# Build optimized release
release:
	cargo build --release

# Run tests
test:
	cargo test

# Clean build artifacts
clean:
	cargo clean
	 rm -rf /tmp/bioswarm*.md

# Install locally (requires cargo install)
install:
	cargo install --path . --force

# Run the CLI
run:
	cargo run

# Run release version
run-release:
	./target/release/bioswarm-v2

# Start API server
serve:
	./target/release/bioswarm-server-v2

# Format code
fmt:
	cargo fmt

# Run lints
lint:
	cargo clippy -- -D warnings

# Check code
check:
	cargo check

# Build Docker image
docker:
	docker build -t bioswarm-v2:latest .

# Full CI pipeline
ci: fmt lint test build
	@echo "✅ All checks passed"

# Quickstart for new users
quickstart:
	@echo "🚀 BioSwarm v2.0 Quickstart"
	@echo ""
	@echo "1. Copy environment file:"
	@echo "   cp .env.sample .env"
	@echo ""
	@echo "2. Edit .env with your API keys"
	@echo ""
	@echo "3. Build and run:"
	@echo "   make release"
	@echo "   make run-release"
	@echo ""
	@echo "4. Check output:"
	@echo "   cat /tmp/bioswarm_v2_report.md"