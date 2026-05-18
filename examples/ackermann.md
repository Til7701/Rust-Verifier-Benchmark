# Ackermann function

```rust
fn ack(m: u64, n: u64) -> u64 {
    if m == 0 {
        n + 1
    } else if n == 0 {
        ack(m - 1, 1)
    } else {
        ack(m - 1, ack(m, n - 1))
    }
}
```

## Source

https://rosettacode.org/wiki/Ackermann_function

### License

GNU Free Document License 1.3
