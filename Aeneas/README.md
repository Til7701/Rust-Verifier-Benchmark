# Aeneas

## Usage

You can verify all examples implemented using Aeneas by running:

```bash
./verify.sh
```

## Termination

In LEAN with

```lean4
termination_by n
decreasing_by scalar_decr_tac
```

## Panics / Unwinding

Modeled as `Result.fail .panic` in return values.
