# Get First Mut

```rust
fn get_mut_fst<A, B>(pair: &mut (A, B)) -> &mut A {
    &mut pair.0
}

fn main() {
    let mut pair = (10, 20);
    let r = get_mut_fst(&mut pair);
    *r = 100;
    assert_eq!(pair, (100i32, 20i32));
}
```

## Comments

### Flux

Cannot index in tuples.

```
error[E0999]: invalid use of refinement parameter
 --> src/get_fst_mut.rs:5:58
  |
5 | #[spec(fn(pair: &mut (A, B)) -> &mut A{result: result == pair.0})]
  |                                                          ^^^^ parameter `pair` refers to a type with no indices
```

```
error[E0999]: assignment might be unsafe
  --> src/get_mut_fst.rs:22:5
   |
22 |     *r = 100;
   |     ^^^^^^^^
```

## Source

https://verus-lang.github.io/verus/guide/mutable-references.html

### License

MIT License

Copyright (c) 2021 The Verus Contributors
