#!/bin/sh
set -e
SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
rm -rf "$SCRIPT_DIR/websrc"
mkdir -p "$SCRIPT_DIR/websrc"
cd "$SCRIPT_DIR/../cube_shuffle-wasm"
trunk build --release --dist "$SCRIPT_DIR/websrc"
