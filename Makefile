.PHONY: help
help: ## Show this help.
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | awk 'BEGIN {FS = ":.*?## "}; {printf "%-10s : %s\n", $$1, $$2}'

.PHONY: lint
lint: ## Code format, lint and unused dependency check.
	cargo fmt --check
	cargo clippy -- -D warnings
	cargo machete

# `cargo machete --fix` exits with 1 even when it removed the unused dependencies
.PHONY: lint-fix
lint-fix: ## Auto-fix formatting, clippy lints and unused dependencies.
	cargo machete --fix || true
	cargo fmt
	cargo clippy --fix --allow-dirty --allow-staged -- -D warnings

.PHONY: test-all
test-all: ## Run all tests.
	cargo test --verbose -- --nocapture
	cd packages/tests/env_always_async_test && cargo test --verbose -- --nocapture
	cd packages/tests/feature_futures_join_test && cargo test --verbose -- --nocapture
	cd packages/tests/wasm_test && \
		cargo test --no-run --verbose --target wasm32-wasip1 && \
		ls target/wasm32-wasip1/debug/deps/*.wasm | xargs -I {} wasmtime {}

# wasm_test is not measured because its tests run on wasmtime.
# Instrumented builds go to a separate target dir, because cargo can not tell them from normal builds.
.PHONY: coverage
coverage: ## Measure test coverage including test packages (requires cargo-llvm-cov).
	export CARGO_TARGET_DIR=$(CURDIR)/target/llvm-cov-target && \
	eval "$$(cargo llvm-cov show-env --sh)" && \
	cargo llvm-cov clean --workspace && \
	cargo test --workspace && \
	cargo test -p portaldi --features multi-thread && \
	(cd packages/tests/env_always_async_test && cargo test) && \
	(cd packages/tests/feature_futures_join_test && cargo test) && \
	cargo llvm-cov report --ignore-filename-regex 'packages/tests/' && \
	cargo llvm-cov report --html --ignore-filename-regex 'packages/tests/'

.PHONY: udeps-all
udeps-all: ## Find unused dependencies in all packages (requires nightly and cargo-udeps).
	cargo +nightly udeps --workspace --all-targets
	cd packages/tests/env_always_async_test && cargo +nightly udeps --all-targets
	cd packages/tests/feature_futures_join_test && cargo +nightly udeps --all-targets
	cd packages/tests/wasm_test && cargo +nightly udeps --all-targets --target wasm32-wasip1

.PHONY: clean-all
clean-all: ## Clean up all packages.
	cargo clean
	cd packages/tests/env_always_async_test && cargo clean
	cd packages/tests/feature_futures_join_test && cargo clean
	cd packages/tests/wasm_test && cargo clean
