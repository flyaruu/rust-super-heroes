#!/usr/bin/env bash
set -euo pipefail


echo "Waiting for http://fights:8000/api/fights/randomfighters ..."
for i in {1..60}; do
  if curl -fsS "http://fights:8000/api/fights/randomfighters" >/dev/null 2>&1; then
    echo "Target is up."
    break
  fi
  sleep 2
done

exec "$@"
