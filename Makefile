.PHONY: build lint

build:
	cargo build

lint:
	cargo check
	cargo clippy
	cargo fmt -- --check

format:
	cargo fmt
