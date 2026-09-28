#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

"${ROOT_DIR}/scripts/build-release.sh"

build_and_push() {
  local image="$1"
  local context="$2"

  docker build --no-cache -t "${image}" "${ROOT_DIR}/${context}"
  docker push "${image}"
}

build_and_push "flyaruu/superheroes-rust-sqlite-rest-heroes:latest" "services/rest-heroes"
build_and_push "flyaruu/superheroes-rust-sqlite-rest-villains:latest" "services/rest-villains"
build_and_push "flyaruu/superheroes-rust-sqlite-grpc-locations:latest" "services/grpc-locations"
build_and_push "flyaruu/superheroes-rust-sqlite-rest-fights:latest" "services/rest-fights"
build_and_push "flyaruu/superheroes-rust-sqlite-k6:latest" "k6-image"
