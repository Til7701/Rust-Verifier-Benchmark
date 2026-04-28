#!/bin/sh

set -e

echo "Building Flux Docker image..."
docker build -t flux:local docker/flux

echo "Verifying Flux Benchmarks..."
docker run --rm --name flux \
  -w /app \
  -u "${UID}":"${GID}" \
  -v "$(pwd)":/app \
  flux:local sh -c "cd Flux && cargo flux"
