.PHONY: clippy
clippy:
	cargo clippy --all -- -D warnings

.PHONY: fmt
fmt:
	cargo +nightly fmt --all

.PHONY: fmt-check
fmt-check:
	cargo +nightly fmt --all -- --check
