#!/usr/bin/env bash

set -ex
cd "$(dirname "$0")/.."
source ./ci/lumos-version.sh install

pnpm install
pnpm build

cd token-swap/js
pnpm build:program
pnpm lint
pnpm test
