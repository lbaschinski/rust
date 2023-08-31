# Returns a full Rust toolchain specified in `rust-toolchain.toml` that is usable in Nix.
# `rust-bin` comes from a rust overlay, see `sources.json`.
{ rust-bin }:
rust-bin.fromRustupToolchainFile ../src/rust-toolchain.toml
