#!/bin/sh

set -e

echo "Verifying Verus Benchmarks..."
docker run --rm --name verus \
  -w /app \
  -u "${UID}":"${GID}" \
  -v "$(pwd)":/app \
  verus:local sh -c "cd Verus && cargo verus verify"
