#!/bin/sh

set -e

echo "Building Verus Docker image..."
docker build -t verus:local .dev/verus
