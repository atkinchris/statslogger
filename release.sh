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
class Statslogger < Formula
  desc "$DESC"
  homepage "https://github.com/$AUTHOR/$NAME"
  url "https://github.com/$AUTHOR/$NAME/releases/download/v$VERSION/$NAME-$VERSION.tar.gz"
  sha256 "$SHA256"
  bottle :unneeded

  plist_options :startup => true
  def plist; <<~EOS
    <?xml version="1.0" encoding="UTF-8"?>
    <!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
    <plist version="1.0">
    <dict>
      <key>Label</key>
        <string>#{plist_name}</string>
      <key>ProgramArguments</key>
      <array>
        <string>#{opt_bin}/statslogger</string>
        <string>--time</string>
        <string>15</string>
        <string>--output</string>
        <string>statslogger.ndjson</string>
      </array>
      <key>RunAtLoad</key>
      <true />
      <key>KeepAlive</key>
      <false />
      <key>WorkingDirectory</key>
      <string>#{var}/log</string>
      <key>StandardOutPath</key>
      <string>/dev/null</string>
      <key>StandardErrorPath</key>
      <string>#{var}/log/statslogger_err.log</string>
    </dict>
    </plist>
    EOS
  end

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
