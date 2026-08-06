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
    cargo fmt && cargo clippy --all-targets --all-features -- -D warnings && cargo test -- --test-threads=1 --nocapture

example:
     cargo run --example types_example