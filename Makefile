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

update-version:
	@release-plz update

update-changelog:
	APP_VERSION="$$(cargo metadata --format-version=1 --no-deps | jq --raw-output '.packages.[0].version')"; \
	grep '^## \['$${APP_VERSION}'\]' CHANGELOG.md >/dev/null 2>&1 \
		&& echo "This version is already in the changelog!" \
		|| sed -e '/^<!-- changes -->$$/r'<( \
		echo -e "\n## [$${APP_VERSION}]\n"; \
		gh api repos/yitsushi/eval-md/releases/generate-notes -F tag_name=$${APP_VERSION} --jq .body \
			| sed -e 's/^#/##/' \
	) -i -- CHANGELOG.md

release-prepare: update-version update-changelog
	git checkout -b release-$((date '+%Y-%m-%d-%s'))
	git add Cargo.lock Cargo.toml CHANGELOG.md
	git commit -m "release: $$(cargo metadata --format-version=1 --no-deps | jq --raw-output '.packages.[0].version')"
	git push -u origin $$(git rev-parse --abbrev-ref HEAD)
	gh pr create --label=release

release:
	@git checkout main
	@git fetch origin/main
	@git reset --hard origin/main
	@release-plz release
