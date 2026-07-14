# Kani

## Usage

You can verify all examples implemented using Kani by running:

```bash
cargo kani -Z function-contracts -Z stubbing
```

## Termination

Checks for termination of loops. You can opt out with the decreases clause `decreases *`.
Does not check termination of recursive functions.

## Panics / Unwinding

Disallows panics.
