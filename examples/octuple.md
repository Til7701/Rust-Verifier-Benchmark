# Octuple

Source: https://verus-lang.github.io/verus/guide/requires_ensures.html

A simple function to multiply a number by 8.

```rust
fn octuple(x1: i8) -> i8 {
    let x2 = x1 + x1;
    let x4 = x2 + x2;
    x4 + x4
}
```

## License

MIT License

Copyright (c) 2021 The Verus Contributors
