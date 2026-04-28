#!/bin/sh

set -e

echo "Building Flux Docker image..."
docker build -t flux:local .dev/flux
