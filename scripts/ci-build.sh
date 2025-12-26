#!/usr/bin/env bash
set -euo pipefail

if ! command -v cargo >/dev/null 2>&1; then
  echo "cargo not found – installing Rust toolchain (minimal profile)..."
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs \
    | sh -s -- -y --profile minimal --default-toolchain stable
  if [ -f "$HOME/.cargo/env" ]; then
    # shellcheck disable=SC1090
    . "$HOME/.cargo/env"
  fi
fi

export PATH="$HOME/.cargo/bin:$PATH"

echo "Running Nx docs build..."
bun run build

if [ -d "apps/docs/.vercel/output" ]; then
  rm -rf .vercel
  mkdir -p .vercel
  cp -R apps/docs/.vercel/output .vercel/
fi
