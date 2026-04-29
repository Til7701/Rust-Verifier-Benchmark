#!/bin/sh

set -e

echo "Building Kani Docker image..."
docker build -t kani:local .dev/kani
