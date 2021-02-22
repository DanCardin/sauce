.PHONY: test build lint format changelog

test:
	cargo test

build:
	cargo build

lint:
	cargo check
	cargo clippy -- -D warnings
	cargo fmt -- --check

format:
	cargo fmt

changelog:
	convco changelog > CHANGELOG.md
