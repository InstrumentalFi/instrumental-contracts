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
	@RUSTFLAGS='-C link-arg=-s' cargo build --lib --release --target wasm32-unknown-unknown --locked --workspace --exclude instrumental-testing

.PHONY: schema
schema:
	@find contracts/* -maxdepth 2 -type f -name Cargo.toml -execdir sh -c '\
    cargo run schema && \
    mkdir -p ../../schemas/$$(basename $$(pwd)) && \
    rm -Rf schema/raw && \
    mv schema/* ../../schemas/$$(basename $$(pwd))/ \
    ' \;
