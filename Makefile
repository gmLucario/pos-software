lint-rust:
	@cargo fmt --version
	@cargo fmt --all -- --check
	@cargo clippy --version
	@cargo clippy --tests -- -D warnings -A incomplete_features -W clippy::dbg_macro -W clippy::print_stdout

upgrade-crates:
	@cargo upgrade && cargo upgrade --manifest-path crates/Cargo.toml  # cargo-edit is required: https://crates.io/crates/cargo-edit
