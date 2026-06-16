# Concat slices to Vec with Copy

Concat two slices of any type implementing `Copy` and return it as a `Vec<T>`.

```rust
fn concat_slices_to_vec_copy<T: Copy>(x: &[T], y: &[T]) -> Vec<T> {
    let mut concat = Vec::with_capacity(x.len() + y.len());
    for i in 0..x.len() {
        concat.push(x[i]);
    }
    for i in 0..y.len() {
        concat.push(y[i]);
    }
    concat
}
```

## Comments

### Creusot

Cannot prove termination due to the loops. https://guide.creusot.rs/v0.11.0/termination.html
