# Getter and Setter in a Struct

```rust
mod m {
    pub struct S {
        value: i32,
    }

    impl S {
        pub fn new() -> Self {
            S { value: 0 }
        }

        pub fn get_value(&self) -> i32 {
            self.value
        }

        pub fn set_value(&mut self, v: i32) {
            self.value = v;
        }
    }
}

use self::m::*;

fn main() {
    let mut s = S::new();

    let value = s.get_value();
    assert!(value == 0);

    s.set_value(5);

    let value = s.get_value();
    assert!(value == 5);
}
```

## Source

https://flux-rs.github.io/flux/tutorial/03-structs.html

### License

