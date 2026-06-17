# Assert

Checks, whether the verifier can handle panics in the code, even though they are never reached. Implementations should
add `b == true` as a precondition and ensure that the method does not panic.

```rust
pub fn assert(b: bool) {
    if !b {
        panic!("assertion failed");
    }
}

fn main() {
    assert(true);
}
```

## Comments

### Verus

```
error: panic is not supported (if you used Rust's `assert!` macro, you may have meant to use Verus's `assert` function)
  --> src/assert.rs:10:9
   |
10 |         panic!();
   |         ^^^^^^^^

error: could not compile `verus-test` (lib) due to 1 previous error
```

## Source

https://flux-rs.github.io/flux/tutorial/02-ownership.html

### License

MIT License
