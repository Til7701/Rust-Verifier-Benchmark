# Concat slices to Vec with Clone

Concat two slices of any type implementing `Clone` and return it as a `Vec<T>`.

```rust
fn concat_slices_to_vec_clone<T: Clone>(x: &[T], y: &[T]) -> Vec<T> {
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

### Creusot

Cannot prove termination due to the loops. https://guide.creusot.rs/v0.11.0/termination.html

### Flux

- Had to use `RVec` from Flux instead of `Vec`.
- Cannot create assertions about the content of the lists, only about the length.
