#!/bin/bash
########################################################
# Script to build and package a release                #
# usage: ./release.sh                                  #
########################################################

# Fail on the first error, rather than continuing
set -e

AUTHOR="atkinchris"
NAME=$(cat Cargo.toml | sed -n -e 's/^.*name = "//p'  | sed -e 's/^"//' -e 's/"$//' | head -n 1)
VERSION=$(cat Cargo.toml | sed -n -e 's/^.*version = "//p'  | sed -e 's/^"//' -e 's/"$//' | head -n 1)
DESC=$(cat Cargo.toml | sed -n -e 's/^.*description = "//p'  | sed -e 's/^"//' -e 's/"$//' | head -n 1)
FILENAME="releases/${NAME}-${VERSION}.tar.gz"

cargo build --release
strip ./target/release/$NAME
rm -rf releases
mkdir -p releases
tar -czf "$FILENAME" --directory=target/release $NAME
SHA256=$(shasum -a 256 $FILENAME | cut -d " " -f 1)

FORMULA=$(cat <<EOM
class StatsLogger < Formula
  desc "$DESC"
  homepage "https://github.com/$AUTHOR/$NAME"
  url "https://github.com/$AUTHOR/$NAME/releases/download/v$VERSION/$NAME-$VERSION.tar.gz"
  sha256 "$SHA256"
  bottle :unneeded
  def install
    bin.install "$NAME"
  end
  test do
    system "#{bin}/$NAME", "--version"
  end
end
EOM
)

echo "$FORMULA" > releases/$NAME.rb

git tag --force "v${VERSION}"
git push --tags
