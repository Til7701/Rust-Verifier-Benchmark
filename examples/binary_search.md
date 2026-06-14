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

Cannot prove absence of panics, since the slices len function does not ensure it.

```
error: cannot show this call will not unwind, in function marked 'no_unwind'
  --> src/binary_search.rs:15:25
   |
15 |     let mut i2: usize = v.len() - 1;
   |                         ^^^^^^^ call to core::slice::impl&%0::len might unwind
   |
  ::: /root/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/vstd-0.0.0-2026-05-31-0205/slice.rs:83:1
   |
83 | pub assume_specification<T>[ <[T]>::len ](slice: &[T]) -> (len: usize)
   | --------------------------------------------------------------------- perhaps you need to mark this function as 'no_unwind'?
```

## Source

https://verus-lang.github.io/verus/guide/binary_search.html#example-binary-search

Slightly modified:

- Vec initialization using macro instead of manual push calls.
- `&[u64]` instead of `&Vec<u64>`

### License

MIT License

Copyright (c) 2021 The Verus Contributors
