# Flux

## Usage

You can verify all examples implemented using Flux by running:

```bash
cargo flux
```

## Termination

Flux does not check for termination and I could not find any documentation on it.

## Panics / Unwinding

Flux does not check for the absence of panics automatically.
You can add the attribute `#[no_panic]` to check for panics.
