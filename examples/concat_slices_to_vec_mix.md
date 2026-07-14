# Concat slices to Vec with mixed methods

Concat two slices of any type implementing `Copy` and return it as a `Vec<T>`.

```rust
fn concat_slices_to_vec_mix<T: Copy>(x: &[T], y: &[T]) -> Vec<T> {
    let mut concat = x.to_vec();

    for e in y {
        concat.push(*e);
    }

    concat
}
```

## Comments

### Aeneas

Does not have specs for range iterators

### Creusot

Cannot prove termination due to the loops. https://guide.creusot.rs/v0.11.0/termination.html

### Flux

- Had to use `RVec` from Flux instead of `Vec`.
- Cannot create assertions about the content of the lists, only about the length.
- Had to activate `scrape_quals` to get invariants. Manual definition of invariants did not work for the second loop.
  See test module to see how to declare invariants.

### Kani

Cannot check all possible inputs due to state space explosion.

### Verus

Using separate vector initialization and calling `extend_from_slice` twice because `to_vec` is not supported out of the
box. This might be solved
with https://verus-lang.github.io/verus/guide/reference-assume-specification.html#assume_specification
