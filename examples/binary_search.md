# Binary Search

```rust
fn binary_search(v: &[u64], k: u64) -> usize {
    let mut i1: usize = 0;
    let mut i2: usize = v.len() - 1;
    while i1 != i2
    {
        let ix = i1 + (i2 - i1) / 2;
        if v[ix] < k {
            i1 = ix + 1;
        } else {
            i2 = ix;
        }
    }
    i1
}
```

## Comments

### Verus

Implemented using `u64` instead of a generic Type. Generic types may be possible, but did not work at first.

## Source

https://verus-lang.github.io/verus/guide/binary_search.html#example-binary-search

Slightly modified:

- Vec initialization using macro instead of manual push calls.
- `&[u64]` instead of `&Vec<u64>`

### License

MIT License

Copyright (c) 2021 The Verus Contributors
