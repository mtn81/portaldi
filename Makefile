.PHONY: help
help: ## Show this help.
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | awk 'BEGIN {FS = ":.*?## "}; {printf "%-10s : %s\n", $$1, $$2}'

.PHONY: check 
check: ## Code format and lint check.
	cargo fmt --check
	cargo clippy -- -D warnings

.PHONY: test-all
test-all: ## Run all tests.
	cargo test
	cd packages/tests/env_always_async_test && cargo test
	cd packages/tests/feature_futures_join_test && cargo test
	cd packages/tests/wasm_test && \
		cargo test --no-run --target wasm32-wasip1 && \
		ls target/wasm32-wasip1/debug/deps/*.wasm | xargs -I {} wasmtime {}

.PHONY: clean-all
clean-all: ## Clean up all packages.
	cargo clean
	cd packages/tests/env_always_async_test && cargo clean
	cd packages/tests/feature_futures_join_test && cargo clean
	cd packages/tests/wasm_test && cargo clean
