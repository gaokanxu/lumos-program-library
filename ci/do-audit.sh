#!/usr/bin/env bash

set -e
cd "$(dirname "$0")/.."
source ./ci/rust-version.sh stable

cargo_audit_ignores=(
  # Potential segfault in the time crate
  #
  # Blocked on chrono updating `time` to >= 0.2.23
  --ignore RUSTSEC-2020-0071

  # tokio: vulnerability affecting named pipes on Windows
  #
  # Exception is a stopgap to unblock CI
  # https://github.com/lumos-labs/lumos/issues/29586
  --ignore RUSTSEC-2023-0001

  # ed25519-dalek: Double Public Key Signing Function Oracle Attack
  #
  # Remove once SPL upgrades to Lumos v1.17 or greater
  --ignore RUSTSEC-2022-0093

  # webpki: CPU denial of service in certificate path building
  #
  # No fixed upgrade is available! Only fix is switching to rustls-webpki
  --ignore RUSTSEC-2023-0052

  # tungstenite
  #
  # Remove once SPL upgrades to Lumos v1.17 or greater
  --ignore RUSTSEC-2023-0065

  # curve25519-dalek
  #
  # Remove once SPL upgrades to curve25519-dalek v4
  --ignore RUSTSEC-2024-0344
)
cargo +"$rust_stable" audit "${cargo_audit_ignores[@]}"
