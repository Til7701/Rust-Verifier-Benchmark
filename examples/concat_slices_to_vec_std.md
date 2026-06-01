# Concat slices to Vec using standard library functions

Concat two slices of any type implementing `Clone` and return it as a `Vec<T>`.

```rust
fn concat_slices_to_vec_std<T: Clone>(x: &[T], y: &[T]) -> Vec<T> {
    let mut concat = x.to_vec();
    concat.extend_from_slice(y);
    concat
}
```

## Comments

### Verus

Using separate vector initialization and calling `extend_from_slice` twice because `to_vec` is not supported out of the
box. This might be solved
with https://verus-lang.github.io/verus/guide/reference-assume-specification.html#assume_specification
