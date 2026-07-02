#!/bin/sh

set -e

charon cargo --preset=aeneas --dest-file=aeneas_test.llbc
# -loops-to-rec https://github.com/AeneasVerif/aeneas/blob/main/documentation/tips-and-tricks.md#loop-translation-prefer--loops-to-rec
aeneas -backend lean aeneas_test.llbc -dest proofs -subdir /AeneasTest/Code -split-files -loops-to-rec -namespace AeneasTest
cd proofs
lake --version
lake build
cd ..
