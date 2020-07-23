#!/bin/sh

set -e

. $(dirname $0)/functions.sh

# only test without coverage in nightly since the span of diagnositcs is
# currently not for the nightly and the other channels, most likely because
# of `proc-macro2`
if [ "$TRAVIS_RUST_VERSION" = "stable" ]; then
	log Installing \`rustfmt\` component for channel $TRAVIS_RUST_VERSION
	rustup component add rustfmt
fi

# --- Setup cargo-tarpaulin ----------------------------------------------------------

LATEST=$(cargo search cargo-tarpaulin | grep cargo-tarpaulin | cut -f2 -d"\"")
log Downloading cargo-tarpaulin v$LATEST
URL="https://github.com/xd009642/tarpaulin/releases/download/${LATEST}/cargo-tarpaulin-${LATEST}-travis.tar.gz"
curl -SsL "$URL" | tar xzvC "$HOME/.cargo/bin"

# --- Setup cargo-cache ------------------------------------------------------

log Installing latest cargo-cache
cargo install -f cargo-cache --no-default-features --features ci-autoclean --root "$HOME/.cargo"
