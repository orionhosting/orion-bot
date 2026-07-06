# local ci
cargo fmt --all --check
cargo clippy --all-targets --locked -- -D warnings