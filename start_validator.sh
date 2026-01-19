#!/bin/bash
# start_validator.sh - Start validator with Geyser plugin

set -e

PLUGIN_DIR="$(pwd)"
CONFIG_PATH="$PLUGIN_DIR/config.json"
DYLIB_PATH="$PLUGIN_DIR/target/release/libtransactionlogger.dylib"

echo "🚀 Starting Validator with Geyser Plugin"
echo "========================================="

# Check files
[ ! -f "$CONFIG_PATH" ] && echo "❌ config.json not found!" && exit 1
[ ! -f "$DYLIB_PATH" ] && echo "❌ Plugin not found! Run: cargo build --release" && exit 1

echo "✅ Config: $CONFIG_PATH"
echo "✅ Plugin: $DYLIB_PATH"
echo ""
echo "Starting validator..."
echo ""

solana-test-validator --geyser-plugin-config "$CONFIG_PATH" --reset
