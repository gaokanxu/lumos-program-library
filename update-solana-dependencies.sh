#!/usr/bin/env bash
#
# Updates the lumos version in all the SPL crates
#

lumos_ver=$1
if [[ -z $lumos_ver ]]; then
  echo "Usage: $0 <new-lumos-version>"
  exit 1
fi

cd "$(dirname "$0")"
source ./ci/lumos-version.sh
old_lumos_ver=${lumos_version#v}

sed -i'' -e "s#lumos_version=v.*#lumos_version=v${lumos_ver}#" ./ci/lumos-version.sh
sed -i'' -e "s#lumos_version = \".*\"#lumos_version = \"${lumos_ver}\"#" ./Anchor.toml

declare tomls=()
while IFS='' read -r line; do tomls+=("$line"); done < <(find . -name Cargo.toml)

crates=(
  lumos-account-decoder
  lumos-banks-client
  lumos-banks-interface
  lumos-banks-server
  lumos-bloom
  lumos-bucket-map
  lumos-clap-utils
  lumos-clap-v3-utils
  lumos-cli-config
  lumos-cli-output
  lumos-client
  lumos-connection-cache
  lumos-core
  lumos-entry
  lumos-faucet
  lumos-frozen-abi
  lumos-frozen-abi-macro
  lumos-geyser-plugin-interface
  lumos-geyser-plugin-manager
  lumos-gossip
  lumos-ledger
  lumos-logger
  lumos-measure
  lumos-merkle-tree
  lumos-metrics
  lumos-net-utils
  lumos-perf
  lumos-poh
  lumos-program-runtime
  lumos-program-test
  lumos-address-lookup-table-program
  lumos-bpf-loader-program
  lumos-compute-budget-program
  lumos-config-program
  lumos-stake-program
  lumos-vote-program
  lumos-zk-token-proof-program
  lumos-pubsub-client
  lumos-quic-client
  lumos-rayon-threadlimit
  lumos-remote-wallet
  lumos-rpc
  lumos-rpc-client
  lumos-rpc-client-api
  lumos-rpc-client-nonce-utils
  lumos-runtime
  lumos-sdk
  lumos-sdk-macro
  lumos-program
  lumos-send-transaction-service
  lumos-storage-bigtable
  lumos-storage-proto
  lumos-streamer
  lumos-test-validator
  lumos-thin-client
  lumos-tpu-client
  lumos-transaction-status
  lumos-udp-client
  lumos-version
  lumos-zk-token-sdk
  lumos-zk-sdk
)

set -x
for crate in "${crates[@]}"; do
  sed -E -i'' -e "s:(${crate} = \")([=<>]*)${old_lumos_ver}([^\"]*)\".*:\1\2${lumos_ver}\3\":" "${tomls[@]}"
  sed -E -i'' -e "s:(${crate} = \{ version = \")([=<>]*)${old_lumos_ver}([^\"]*)(\".*):\1\2${lumos_ver}\3\4:" "${tomls[@]}"
done
