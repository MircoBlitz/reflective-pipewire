#!/bin/bash
set -e

PLUGIN_DIR="dist/de.mircoblitz.reflective-pipewire.sdPlugin"

cargo build --release

rm -rf dist
mkdir -p "$PLUGIN_DIR/x86_64-unknown-linux-gnu"

cp target/release/reflective-pipewire "$PLUGIN_DIR/x86_64-unknown-linux-gnu/"
cp manifest.json "$PLUGIN_DIR/"
cp -r assets/icons "$PLUGIN_DIR/"
cp -r propertyInspector "$PLUGIN_DIR/"

cd dist
zip -r reflective-pipewire.zip de.mircoblitz.reflective-pipewire.sdPlugin/
