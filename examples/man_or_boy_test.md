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
    if k <= 0 { x4() + x5() } else { b() }
}

fn main() {
    let result = a(10, &|| 1, &|| -1, &|| -1, &|| 1, &|| 0);
    assert_eq!(result, -67);
}
```

## Comments

### Aeneas

```
[Error] Detected groups of mixed mutually recursive definitions (such as a type mutually recursive with a function, or a function mutually recursive with a trait implementation):

# Group 1:
- fun decl(Source: 'src/test.rs', lines 3:0-23:1): aeneas_test::test::a
- trait impl(Source: 'src/test.rs', lines 16:12-19:9): aeneas_test::test::a::{core::ops::function::FnOnce<(&'_ (aeneas_test::test::a::B<'_>)), i32> for aeneas_test::test::a::closure<0, 1, 2, 3, 4, 5, 6, 7, 8>}
- trait impl(Source: 'src/test.rs', lines 16:12-19:9): aeneas_test::test::a::{core::ops::function::FnMut<(&'_ (aeneas_test::test::a::B<'_>)), i32> for aeneas_test::test::a::closure<0, 1, 2, 3, 4, 5, 6, 7, 8>}
- trait impl(Source: 'src/test.rs', lines 16:12-19:9): aeneas_test::test::a::{core::ops::function::Fn<(&'_ (aeneas_test::test::a::B<'_>)), i32> for aeneas_test::test::a::closure<0, 1, 2, 3, 4, 5, 6, 7, 8>}
- fun decl(Source: 'src/test.rs', lines 16:12-19:9): aeneas_test::test::a::{core::ops::function::FnOnce<(&'_ (aeneas_test::test::a::B<'_>)), i32> for aeneas_test::test::a::closure<0, 1, 2, 3, 4, 5, 6, 7, 8>}::call_once
- fun decl(Source: 'src/test.rs', lines 16:12-19:9): aeneas_test::test::a::{core::ops::function::FnMut<(&'_ (aeneas_test::test::a::B<'_>)), i32> for aeneas_test::test::a::closure<0, 1, 2, 3, 4, 5, 6, 7, 8>}::call_mut
- fun decl(Source: 'src/test.rs', lines 16:12-19:9): aeneas_test::test::a::{core::ops::function::Fn<(&'_ (aeneas_test::test::a::B<'_>)), i32> for aeneas_test::test::a::closure<0, 1, 2, 3, 4, 5, 6, 7, 8>}::call

[Error] Mixed declaration groups (which contain both type and function declarations or functions and traits, for instance) are not supported yet: [13, 13, 14, 15, 240, 241, 242]
[Error] Dynamic trait types are not supported yet
```

### Creusot

```
error: forbidden dyn type: dyn std::ops::Fn() -> i32
```

### Flux

```
error: internal compiler error: /opt/flux/crates/flux-middle/src/rty/binder.rs:193:18: unexpected escaping region BoundRegion { var: 0, kind: BrNamed(DefId(0:196 ~ flux_test[aed3]::man_or_boy_test::a::B::'_)) }
  --> src/man_or_boy_test.rs:28:36
   |
28 |             return a(k1.get(), &|| (b.f)(b), x1, x2, x3, x4);
   |                                    ^^^^^


thread 'rustc' (6006) panicked at /opt/flux/crates/flux-middle/src/rty/binder.rs:193:18:
Box<dyn Any>
```

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
