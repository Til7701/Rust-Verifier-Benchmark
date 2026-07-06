# Range

```rust
mod m {
    pub struct Range {
        start: i32,
        end: i32,
    }

    impl Range {
        pub fn new(start: i32, end: i32) -> Self {
            Range { start, end }
        }

        pub fn get_start(&self) -> i32 {
            self.start
        }

        pub fn set_start(&mut self, start: i32) {
            self.start = start;
        }

        pub fn get_end(&self) -> i32 {
            self.end
        }

        pub fn set_end(&mut self, end: i32) {
            self.end = end;
        }
    }
}

use m::*;

fn main() {
    let mut range = Range::new(0, 1);
    let start = range.get_start();
    assert_eq!(start, 0);
    let end = range.get_end();
    assert_eq!(end, 1);

    range.set_start(-2);

    let start = range.get_start();
    assert_eq!(start, -2);
    let end = range.get_end();
    assert_eq!(end, 1);

    range.set_end(42);

    let start = range.get_start();
    assert_eq!(start, -2);
    let end = range.get_end();
    assert_eq!(end, 42);
}
```

## Source

https://flux-rs.github.io/flux/tutorial/03-structs.html
