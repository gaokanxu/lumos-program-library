#!/usr/bin/env bash

set -ex
cd "$(dirname "$0")/.."
source ./ci/lumos-version.sh install

cd stake-pool/py
python3 -m venv venv
source ./venv/bin/activate
pip3 install -r requirements.txt
pip3 install -r optional-requirements.txt
check_dirs=(
  "bot"
  "spl_token"
  "stake"
  "stake_pool"
  "system"
  "tests"
  "vote"
)
flake8 "${check_dirs[@]}"
mypy "${check_dirs[@]}"
python3 -m pytest
