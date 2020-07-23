#!/bin/sh -e

. $(dirname $0)/functions.sh

# --- Test without coverage --------------------------------------------------

# only test without coverage in nightly since the span of diagnositcs is
# currently not for the nightly and the other channels, most likely because
# of `proc-macro2`
if [ "$TRAVIS_RUST_VERSION" = "nightly" ]; then
	log Testing without measuring coverage
	cargo test
fi


# --- Test without coverage --------------------------------------------------

log Testing with coverage
cargo tarpaulin --lib -v --out Xml --ciserver travis-ci


# --- Check code format ------------------------------------------------------

# only test without coverage in nightly since the span of diagnositcs is
# currently not for the nightly and the other channels, most likely because
# of `proc-macro2`
if [ "$TRAVIS_RUST_VERSION" = "stable" ]; then
	log Checking Rust code format with \`cargo fmt\`
	cargo fmt -- --check
fi
