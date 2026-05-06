#!/bin/sh

set -e

echo "Verifying Aeneas Examples"
cd Aeneas
charon cargo --preset=aeneas --dest-file=aeneas_test.llbc
aeneas -backend lean aeneas_test.llbc -dest proofs -subdir /AeneasTest/Code -split-files -namespace AeneasTest
cd proofs && lake build
cd ../..
echo "Successfully verified Aeneas Examples"

echo "Verifying Creusot Examples"
cd Creusot
cargo creusot init
cargo creusot prove
cd ..
echo "Successfully verified Creusot Examples"

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
