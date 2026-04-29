#!/bin/sh

set -e

echo "Verifying Kani Benchmarks..."
docker run --rm --name kani \
  -w /app \
  -u "${UID}":"${GID}" \
  -v "$(pwd)":/app \
  kani:local sh -c "cd Kani && cargo kani"
