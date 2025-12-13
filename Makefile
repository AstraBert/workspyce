.PHONY: build test

build:
	cargo build --target-dir target/

test:
	cargo test