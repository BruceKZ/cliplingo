#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
APP_DIR="$ROOT_DIR/apps/desktop"
TAURI_DIR="$APP_DIR/src-tauri"

if [ -f "$HOME/.cargo/env" ]; then
  # shellcheck disable=SC1090
  source "$HOME/.cargo/env"
fi

pnpm --dir "$APP_DIR" lint
pnpm --dir "$APP_DIR" typecheck
pnpm --dir "$APP_DIR" build
cargo test --manifest-path "$TAURI_DIR/Cargo.toml"
cargo check --manifest-path "$TAURI_DIR/Cargo.toml"
