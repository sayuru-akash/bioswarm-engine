build:
	cargo build --release

test:
	cargo test

run:
	cargo run -- run --query "ai market intelligence"

fmt:
	cargo fmt

clippy:
	cargo clippy --all-targets --all-features -- -D warnings
