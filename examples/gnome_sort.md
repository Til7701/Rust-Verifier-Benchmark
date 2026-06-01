# Gnome Sort

Implementation of the [Gnome Sort](https://en.wikipedia.org/wiki/Gnome_sort) algorithm.

```rust
fn gnome_sort<T: Ord>(v: &mut [T]) {
    let mut i = 0;
    while i < v.len() {
        if i == 0 || v[i - 1] <= v[i] {
            i += 1;
        } else {
            v.swap(i - 1, i);
            i -= 1;
        }
    }
}
```

## Source

Xavier Denis, Jacques-Henri Jourdan, Claude Marché. Creusot: a Foundry for the Deductive Verification
of Rust Programs. ICFEM 2022 - 23th International Conference on Formal Engineering Methods, Oct 2022,
Madrid, Spain. ⟨hal-03737878⟩

(Updated to use new API: https://github.com/creusot-rs/tutorial/blob/main/src/solutions/ex1_gnome_sort.rs)

## Comments

### Verus

```
error: complex arguments to &mut parameters are currently unsupported
```

TODO: Try with `Vec`
