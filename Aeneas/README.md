# Aeneas

## Usage

You can verify all examples implemented using Aeneas by running:

```bash
charon cargo --preset=aeneas --dest-file=aeneas_test.llbc
aeneas -backend lean aeneas_test.llbc -dest proofs -subdir /AeneasTest/Code -split-files -namespace AeneasTest
cd proofs
lake build
cd ..
```

## Termination

TODO

## Panics / Unwinding

TODO
