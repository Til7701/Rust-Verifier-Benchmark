# Triangle Recursive

```rust
fn triangle_rec(n: u64) -> u64 {
    if n == 0 {
        0
    } else {
        n + triangle_rec(n - 1)
    }
}
```

## Comments

### Kani

Recursive functions cannot have decreases clauses. Thus, Kani cannot prove termination and unwinds the call
infinitely.

https://model-checking.github.io/kani/reference/experimental/loop-contracts.html#decreases-clause-limitations-and-comparison-with-other-tools
