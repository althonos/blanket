#!/bin/sh -e

. $(dirname $0)/functions.sh

# --- Deploy crate -----------------------------------------------------------

log Deploying \`blanket\` $TRAVIS_TAG
cargo publish --manifest-path Cargo.toml --token $CRATES_IO_TOKEN
