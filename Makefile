SHELL = /bin/sh

.PHONY: check
check:
	@cargo check

.PHONY: clippy
clippy:
	@cargo clippy --tests -- -D warnings

.PHONY: fmt
fmt:
	@cargo +nightly fmt --all -- --check

.PHONY: test
test:
	@cargo test

.PHONY: clean
clean:
	@cargo clean

.PHONY: build
build:
	@RUSTFLAGS='-C link-arg=-s' cargo build --release --target wasm32-unknown-unknown --locked --workspace --exclude instrumental-testing

.PHONY: schema
schema:
	@cargo run --example schema
