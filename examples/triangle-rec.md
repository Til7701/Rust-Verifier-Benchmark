# Triangle Recursive

```rust
fn triangle_rec(n: u64) -> u64 {
    if n == 0 {
        0
    } else {
        n + triangle(n - 1)
    }
}
```
