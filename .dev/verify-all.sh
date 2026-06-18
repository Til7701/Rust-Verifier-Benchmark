#!/bin/sh

set -e

echo "Verifying Aeneas Examples"
cd Aeneas
cargo --version
charon version
aeneas -version
charon cargo --preset=aeneas --dest-file=aeneas_test.llbc
aeneas -backend lean aeneas_test.llbc -dest proofs -subdir /AeneasTest/Code -split-files -namespace AeneasTest
cd proofs
lake --version
lake build
cd ../..
echo "Successfully verified Aeneas Examples\n\n"

echo "Verifying Creusot Examples"
cd Creusot
cargo --version
cargo creusot init
cargo creusot prove
cd ..
echo "Successfully verified Creusot Examples\n\n"

echo "Verifying Flux Examples"
cd Flux
cargo --version
cargo flux --version
cargo flux
cd ..
echo "Successfully verified Flux Examples\n\n"

echo "Verifying Kani Examples"
cd Kani
cargo --version
cargo kani --version
cargo kani -Z function-contracts -Z stubbing
cd ..
echo "Successfully verified Kani Examples\n\n"

echo "Verifying VeriFast Examples"
cd VeriFast
cargo --version
cargo verifast
cd ..
echo "Successfully verified VeriFast Examples\n\n"

echo "Verifying Verus Examples"
cd Verus
cargo --version
cargo verus verify
cd ..
echo "Successfully verified Verus Examples\n\n"

echo "Successfully verified all Examples"
