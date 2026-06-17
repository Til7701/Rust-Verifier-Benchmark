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

### Creusot

```
warning: calling external function `extend_from_slice` with no contract will yield an impossible precondition
 --> src/concat_slices_to_vec_std.rs:9:5
  |
9 |     concat.extend_from_slice(y);
  |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^ function called here
  |
  = note: `#[warn(creusot::contractless_external_function)]` on by default

warning: `creusot-test` (lib) generated 1 warning
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.46s
Goal Coma.vc_concat_slices_to_vec_T: ✘ (2/3)
Error: 1 unproved file
Error: 'why3find prove' failed
```

### Flux

- Had to use `RVec` from Flux instead of `Vec`.
- Cannot create assertions about the content of the lists, only about the length.

### Verus

Using separate vector initialization and calling `extend_from_slice` twice because `to_vec` is not supported out of the
box. This might be solved
with https://verus-lang.github.io/verus/guide/reference-assume-specification.html#assume_specification
