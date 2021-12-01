SHELL:=/bin/bash

.DEFAULT_GOAL := default
.PHONY: check fix format lint build build-release test run default clean

check:
	cargo check

fix:
	cargo fix --allow-staged

format:
	cargo fmt

lint:
	cargo fmt -- --check
	cargo clippy
	-cargo audit

build: lint
	cargo build

build-release: lint
	cargo build --release

test: build
	cargo test

run: build-release
	i=1 ; while [[ $$i -le 25 ]] ; do \
		if [ -f "./src/bin/$$i.rs" ]; then \
			cargo run --release --bin $$i ; \
		fi ; \
		((i = i + 1)) ; \
	done

default: run

clean:
	cargo clean