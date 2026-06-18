# Verus

## Usage

You can verify all examples implemented using Verus by running:

```bash
cargo verus verify
```

## Termination

Verus guarantees that functions terminate, if all called functions also terminate.
See: https://verus-lang.github.io/verus/guide/exec_termination.html#lightweight-termination-checking

You can opt out of this by using the attribute `#![verifier::exec_allows_no_decreases_clause]`.

## Panics / Unwinding

Exec functions may unwind by default. However, common causes (like arithmetic overflow) are disallowed.
See: https://verus-lang.github.io/verus/guide/reference-unwind-sig.html#unwinding-signature

You can opt out by adding `no_unwind` or `no_unwind when <expr>` to a function.
