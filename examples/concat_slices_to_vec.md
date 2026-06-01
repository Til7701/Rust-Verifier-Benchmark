# Concat Slices to Vec

A simple function to multiply a number by 8.

```rust
fn concat_slices_to_vec<T: Clone>(x: &[T], y: &[T]) -> Vec<T>
{
    let mut concat = Vec::with_capacity(x.len() + y.len());

    for i in 0..x.len() {
        concat.push(x[i].clone());
    }

    for i in 0..y.len() {
        concat.push(y[i].clone());
    }

    concat
}
```

## Comments

### Verus

Implemented using `Copy` instead of `Clone`. `Clone` may be possible, but did not work at first.
