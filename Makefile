CARGO  = $(or $(shell which cargo),  $(HOME)/.cargo/bin/cargo)
RUSTUP = $(or $(shell which rustup), $(HOME)/.cargo/bin/rustup)

RUST_TOOLCHAIN := $(shell cat rust-toolchain)

build/%:
	$(CARGO) build \
	        --release $(filter-out --release, $(CARGO_FLAGS)) \
	        --package $* \
	        --target wasm32-unknown-unknown
	cp target/wasm32-unknown-unknown/release/$(shell echo $* | sed "s/-/_/g").wasm tests/wasm

.PHONY: test
test: build/swap-install
	$(CARGO) test $(CARGO_FLAGS) --manifest-path "tests/Cargo.toml" --features "use-system-contracts" -- --ignored --nocapture

.PHONY: check-format
check-format:
	$(CARGO) fmt --all -- --check

.PHONY: lint
lint:
	$(CARGO) clippy --all-targets --all -- -D warnings -A renamed_and_removed_lints

.PHONY: setup
setup: rust-toolchain
	$(RUSTUP) update
	$(RUSTUP) toolchain install $(RUST_TOOLCHAIN)
	$(RUSTUP) target add --toolchain $(RUST_TOOLCHAIN) wasm32-unknown-unknown
