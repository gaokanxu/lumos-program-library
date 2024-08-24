#
# This file maintains the lumos versions for use by CI.
#
# Obtain the environment variables without any automatic updating:
#   $ source ci/lumos-version.sh
#
# Obtain the environment variables and install update:
#   $ source ci/lumos-version.sh install

# Then to access the lumos version:
#   $ echo "$lumos_version"
#

if [[ -n $LUMOS_VERSION ]]; then
  lumos_version="$LUMOS_VERSION"
else
  # This file is now out of sync with the versions in Cargo.toml.
  # https://github.com/lumos-labs/lumos-program-library/pull/6182
  # This will require some manual cleanup the next time the version is updated.
  lumos_version=v2.0.0
fi

export lumos_version="$lumos_version"
export PATH="$HOME"/.local/share/lumos/install/active_release/bin:"$PATH"

if [[ -n $1 ]]; then
  case $1 in
  install)
    sh -c "$(curl -sSfL https://release.anza.xyz/$lumos_version/install)"
    lumos --version
    ;;
  *)
    echo "lumos-version.sh: Note: ignoring unknown argument: $1" >&2
    ;;
  esac
fi
