#!/usr/bin/env bash

set -e

# Insist repository is clean
git diff-index --quiet HEAD

version=$(grep "version = " setup.cfg)
version=${version/version = }
version=${version/\'/}
version=${version/\'/}
version=${version/\"/}
version=${version/\"/}

echo "Pushing release-v"$version

git tag -d release-v$version || true
git push origin :release-v$version || true
git tag release-v$version
git push origin release-v$version
