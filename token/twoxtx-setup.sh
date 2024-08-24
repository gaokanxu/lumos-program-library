#!/usr/bin/env bash
#
# Patch in a Lumos monorepo that supports 2x transactions for testing the
# SPL Token 2022 Confidential Transfer extension
#

set -e

here="$(dirname "$0")"
cd "$here"

if [[ ! -d twoxtx-lumos ]]; then
  if [[ -n $CI ]]; then
    git config --global user.email "you@example.com"
    git config --global user.name "Your Name"
    git clone https://github.com/anza-xyz/agave.git twoxtx-lumos
  else
    git clone git@github.com:anza-xyz/agave.git twoxtx-lumos
  fi
fi

if [[ ! -f twoxtx-lumos/.twoxtx-patched ]]; then
  git -C twoxtx-lumos am -3 "$PWD"/twoxtx.patch
  touch twoxtx-lumos/.twoxtx-patched
fi

../patch.crates-io.sh twoxtx-lumos

exit 0
