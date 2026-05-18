# Remainder Recursive

```rust
fn remainder_rec(n: u64, d: u64) -> (u64, u64) {
    if n < d {
        (0, n)
    } else {
        let (q, r) = remainder_rec(n - d, d);
        (q + 1, r)
    }
}
```
