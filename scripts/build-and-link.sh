#!/usr/bin/env bash
set -euo pipefail

TARGET=${1:-aarch64-apple-darwin}

BIN_NAME="motion-core"
CRATE_BIN_NAME="motion-core-cli"
REPO_ROOT="$(cd "$(dirname "$0")/.." && pwd)"
CLI_DIR="$REPO_ROOT/motion-core-cli"
DIST_DIR="$CLI_DIR/js/dist/$TARGET"
DEST_PATH="$DIST_DIR/$BIN_NAME"

if [[ "$TARGET" == *"windows"* ]]; then
  DEST_PATH+=".exe"
fi

echo "Building Rust CLI (target: $TARGET)..."
cargo build \
  --manifest-path "$REPO_ROOT/Cargo.toml" \
  --package motion-core-cli \
  --release \
  --target "$TARGET" \
  --target-dir "$CLI_DIR/target"

mkdir -p "$DIST_DIR"

SOURCE_BUILT="$CLI_DIR/target/$TARGET/release/$CRATE_BIN_NAME"
FALLBACK_BUILT="$CLI_DIR/target/release/$CRATE_BIN_NAME"
if [[ "$TARGET" == *"windows"* ]]; then
  SOURCE_BUILT+=".exe"
  FALLBACK_BUILT+=".exe"
fi

if [[ -f "$SOURCE_BUILT" ]]; then
  cp "$SOURCE_BUILT" "$DEST_PATH"
  echo "Copied binary from $SOURCE_BUILT to $DEST_PATH"
elif [[ -f "$FALLBACK_BUILT" ]]; then
  cp "$FALLBACK_BUILT" "$DEST_PATH"
  echo "Target-specific build not found; copied host binary from $FALLBACK_BUILT to $DEST_PATH"
else
  echo "Built binary not found at $SOURCE_BUILT or $FALLBACK_BUILT"
  exit 1
fi

pushd "$CLI_DIR/js" >/dev/null

echo "Unlinking any global @motion-core/cli..."
npm uninstall -g @motion-core/cli || true

echo "Linking local package..."
npm link

popd >/dev/null

echo "Done. Use 'motion-core --help' to test."
