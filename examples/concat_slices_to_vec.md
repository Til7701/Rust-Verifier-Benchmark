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
