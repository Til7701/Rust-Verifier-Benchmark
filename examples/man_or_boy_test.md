# Man or Boy test

```rust
use std::cell::Cell;

fn a(
    k: i32,
    x1: &dyn Fn() -> i32,
    x2: &dyn Fn() -> i32,
    x3: &dyn Fn() -> i32,
    x4: &dyn Fn() -> i32,
    x5: &dyn Fn() -> i32,
) -> i32 {
    let k1 = Cell::new(k);
    struct B<'a> {
        f: &'a dyn Fn(&B) -> i32,
    }
    let b = B {
        f: &|b| {
            k1.set(k1.get() - 1);
            return a(k1.get(), &|| (b.f)(b), x1, x2, x3, x4);
        },
    };
    let b = || (b.f)(&b);
    return if k <= 0 { x4() + x5() } else { b() };
}
```

## Comments

### Verus

TODO: Check again after Verus update

```
error: The verifier does not yet support the following Rust feature: dyn with more that one trait
 --> src/man_or_boy_test.rs:8:5
  |
8 |     x1: &dyn Fn() -> i32,
  |     ^^^^^^^^^^^^^^^^^^^^

error: The verifier does not yet support the following Rust feature: dyn with more that one trait
  --> src/man_or_boy_test.rs:17:5
   |
17 | /     struct B<'a> {
18 | |         f: &'a dyn Fn(&B) -> i32,
19 | |     }
   | |_____^

error: could not compile `verus-test` (lib) due to 2 previous errors
```

## Source

https://rosettacode.org/wiki/Man_or_boy_test

### License

GNU Free Document License 1.3
