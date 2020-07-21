#!/bin/sh -e

. $(dirname $0)/functions.sh

# --- Test without coverage --------------------------------------------------

log Testing without measuring coverage
cargo test 


# --- Test without coverage --------------------------------------------------

log Testing with coverage
cargo tarpaulin -v --out Xml --ciserver travis-ci
