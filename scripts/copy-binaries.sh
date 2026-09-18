#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

copy_binary() {
  local service="$1"
  local source="${ROOT_DIR}/target/release/${service}"
  local destination_dir="${ROOT_DIR}/services/${service}/binary"
  local destination="${destination_dir}/${service}"

  if [[ ! -x "${source}" ]]; then
    echo "Missing release binary: ${source}" >&2
    echo "Run 'cargo build --release' from the workspace root first." >&2
    exit 1
  fi

  mkdir -p "${destination_dir}"
  cp "${source}" "${destination}"
}

copy_binary "rest-heroes"
copy_binary "rest-villains"
copy_binary "grpc-locations"
copy_binary "rest-fights"
