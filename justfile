# https://just.systems

default:
    echo 'Hello, world!'

run:
	cargo fmt && cargo check && cargo run -q

doc:
	cargo doc --no-deps --open

tree:
    cargo-modules structure

clippy:
	cargo clippy --all-targets --all-features -- -D warnings

test:
    cargo test -- --test-threads=1 --nocapture