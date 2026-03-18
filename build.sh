#!/bin/bash
set -e

PLUGIN_DIR="dist/de.mircoblitz.reflective-pipewire.sdPlugin"

# Bump minor version in Cargo.toml and manifest.json
bump_minor() {
  local file=$1 pattern=$2
  local ver=$(grep -oP "$pattern\K[0-9]+\.[0-9]+\.[0-9]+" "$file" | head -1)
  local major=$(echo "$ver" | cut -d. -f1)
  local minor=$(echo "$ver" | cut -d. -f2)
  local patch=$(echo "$ver" | cut -d. -f3)
  local new="$major.$minor.$((patch + 1))"
  sed -i "s/$ver/$new/" "$file"
  echo "$new"
}

VERSION=$(bump_minor Cargo.toml 'version = "')
bump_minor manifest.json '"Version": "' > /dev/null
echo "Building v$VERSION"

cargo build --release

rm -rf dist
mkdir -p "$PLUGIN_DIR/x86_64-unknown-linux-gnu"

cp target/release/reflective-pipewire "$PLUGIN_DIR/x86_64-unknown-linux-gnu/"
cp manifest.json "$PLUGIN_DIR/"
cp -r assets/icons "$PLUGIN_DIR/"
cp -r propertyInspector "$PLUGIN_DIR/"

cd dist
zip -r reflective-pipewire.zip de.mircoblitz.reflective-pipewire.sdPlugin/
