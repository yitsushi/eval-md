ROOT_DIR := $(shell pwd)

build:
	cargo build --release

clippy:
	cargo clippy -- -Dwarnings

test:
	cargo test

quality-check: test clippy

docs:
	cargo clean --doc
	cargo doc --no-deps

open_docs:
	cargo clean --doc
	cargo doc --no-deps --open

codecov:
	@rm -rf $(ROOT_DIR)/target/cov/
	@mkdir -p $(ROOT_DIR)/target/cov/
	CARGO_INCREMENTAL=0 \
		RUSTFLAGS='-Cinstrument-coverage' \
		LLVM_PROFILE_FILE="$(ROOT_DIR)/target/cov/cargo-test-%p-%m.profraw" \
		cargo test --profile=codecov

	grcov . \
		-s $(ROOT_DIR)/ \
		--binary-path ./target/codecov/ \
		-t html \
		--branch \
		--ignore-not-existing \
		-o $(ROOT_DIR)/target/codecov/coverage/

	xdg-open $(ROOT_DIR)/target/codecov/coverage/index.html
