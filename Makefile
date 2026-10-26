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

.PHONY: coverage
coverage: ## Measure test coverage of the workspace (requires cargo-llvm-cov).
	cargo llvm-cov --workspace --no-report
	cargo llvm-cov report
	cargo llvm-cov report --html

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
