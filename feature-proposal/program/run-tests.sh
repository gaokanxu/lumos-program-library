#!/usr/bin/env bash

set -ex
cd "$(dirname "$0")"
cargo fmt -- --check
cargo clippy
cargo build
cargo build-sbf

if [[ $1 = -v ]]; then
  export RUST_LOG=lumos=debug
fi

cargo test
cargo test-sbf
