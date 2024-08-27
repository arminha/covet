#!/bin/bash

set -e

# Check that the working directory is clean
if ! (git diff --exit-code && git diff --cached --exit-code)
then
	echo "There are uncommitted changes. Abort release.."
	exit 1
fi

# Bump version and create release commit
cargo set-version --workspace --bump $1

VERSION=`cargo metadata --format-version 1 --no-deps | python3 -c "import sys, json; print(json.load(sys.stdin)['packages'][0]['version'])"`

git commit -a -m "Release $VERSION"

# Tag the release
git tag -s -m "Release $VERSION" v$VERSION

