#!/bin/sh

set -e

charon cargo --preset=aeneas --dest-file=aeneas_test.llbc
aeneas -backend lean aeneas_test.llbc -dest proofs -subdir /AeneasTest/Code -split-files -namespace AeneasTest
cd proofs
lake --version
lake build
cd ..
