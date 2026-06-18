# Creusot

## Usage

You can verify all examples implemented using Creusot by running:

```bash
cargo creusot prove
```

## Termination

Creusot does not check for termination of non-ghost functions by default.
See: https://guide.creusot.rs/termination.html

You can check for termination by adding the attribute `#[check(terminates)]`.

## Panics / Unwinding

Creusot checks for most sources of panics automatically and disallows them. However, some panics are allowed:
https://guide.creusot.rs/limitations.html#some-panics-are-allowed
