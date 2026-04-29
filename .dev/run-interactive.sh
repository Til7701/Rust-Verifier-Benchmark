#!/bin/sh

set -e

docker run --rm --name rust-verifiers \
  -w /app \
  -u "${UID}":"${GID}" \
  -v "$(pwd)":/app \
  -it \
  rust-verifiers:local
