# Even Odd with mutual recursion

```rust
fn is_even(n: u64) -> bool {
    if n == 0 {
        return true;
    }
    is_odd(n - 1)
}

fn is_odd(n: u64) -> bool {
    if n == 0 {
        return false;
    }
    is_even(n - 1)
}
```
