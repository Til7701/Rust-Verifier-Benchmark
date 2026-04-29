#!/bin/sh

set -e

echo "Verifying Flux Examples"
cd Flux
cargo flux
cd ..
echo "Successfully verified Flux Examples"

echo "Verifying Kani Examples"
cd Kani
cargo kani
cd ..
echo "Successfully verified Kani Examples"

echo "Verifying VeriFast Examples"
cd VeriFast
cargo verifast
cd ..
echo "Successfully verified VeriFast Examples"

echo "Verifying Verus Examples"
cd Verus
cargo verus verify
cd ..
echo "Successfully verified Verus Examples"

echo "Successfully verified all Examples"
