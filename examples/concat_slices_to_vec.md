# Concat slices to Vec with Clone

Concat two slices of any type implementing `Clone` and return it as a `Vec<T>`.

```rust
fn concat_slices_to_vec<T: Clone>(x: &[T], y: &[T]) -> Vec<T> {
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

### Verus

Cannot prove absence of panics, since the Vec functions do not ensure it.
