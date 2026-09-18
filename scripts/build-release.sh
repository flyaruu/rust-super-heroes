#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

cargo build --release --manifest-path "${ROOT_DIR}/Cargo.toml"
"${ROOT_DIR}/scripts/copy-binaries.sh"
