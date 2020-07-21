#!/bin/sh -e

. $(dirname $0)/functions.sh

# --- Upload coverage to Codecov.io ------------------------------------------

log Uploading coverage statistics to codecov.io
curl -SsL "https://codecov.io/bash" | bash -s

# --- Update GitHub release notes --------------------------------------------

export GEM_PATH="$(ruby -r rubygems -e 'puts Gem.user_dir')"
export PATH="${GEM_PATH}/bin:$PATH"

log Installing chandler gem
gem install --user-install 'faraday:<0.16' chandler

log Updating GitHub release notes
chandler push --github="$TRAVIS_REPO_SLUG" --changelog="CHANGELOG.md"
