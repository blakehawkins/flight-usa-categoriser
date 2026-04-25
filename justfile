default:
    just --list

check: update cargo-check test clippy fmt update outdated

cargo-check:
    cargo check

test:
    cargo test

clippy:
    cargo clippy -- -D warnings

fmt:
    cargo fmt

update:
    cargo update

outdated:
    cargo outdated -R
