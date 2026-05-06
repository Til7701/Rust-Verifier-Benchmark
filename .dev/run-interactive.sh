#!/bin/sh

set -e

docker run --rm --name rust-verifiers \
  -w /app \
  -v "$(pwd)":/app \
  -it \
  rust-verifiers:local
