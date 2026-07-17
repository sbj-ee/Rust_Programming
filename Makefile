.PHONY: all build test fmt clippy lint clean

# Every exercise is declared as a [[bin]] target in Cargo.toml pointing at
# exercises/<NN_name>/main.rs. `cargo build` builds them all in one pass — no
# per-exercise Makefile is needed the way C/C++ need one for link flags.

all: build

build:
	cargo build

test:
	cargo test

fmt:
	cargo fmt --check

clippy:
	cargo clippy --all-targets -- -D warnings

lint: fmt clippy

clean:
	cargo clean
