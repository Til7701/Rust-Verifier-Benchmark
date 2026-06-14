# Ternary Logic

```rust
use std::ops::{BitAnd, BitOr, Not};

#[derive(Copy, Clone, PartialEq, Eq)]
enum Trit {
    True,
    Maybe,
    False,
}

impl Not for Trit {
    type Output = Self;
    fn not(self) -> Self {
        match self {
            Trit::True => Trit::False,
            Trit::Maybe => Trit::Maybe,
            Trit::False => Trit::True,
        }
    }
}

impl BitAnd for Trit {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        match (self, other) {
            (Trit::True, Trit::True) => Trit::True,
            (Trit::False, _) | (_, Trit::False) => Trit::False,
            _ => Trit::Maybe,
        }
    }
}

impl BitOr for Trit {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        match (self, other) {
            (Trit::True, _) | (_, Trit::True) => Trit::True,
            (Trit::False, Trit::False) => Trit::False,
            _ => Trit::Maybe,
        }
    }
}

impl Trit {
    fn imp(self, other: Self) -> Self {
        match self {
            Trit::True => other,
            Trit::Maybe => {
                if let Trit::True = other {
                    Trit::True
                } else {
                    Trit::Maybe
                }
            }
            Trit::False => Trit::True,
        }
    }

    fn eqv(self, other: Self) -> Self {
        match self {
            Trit::True => other,
            Trit::Maybe => Trit::Maybe,
            Trit::False => !other,
        }
    }
}
```

## Comments

### Verus

The not operator `!` seems to crash rustc.

The implementation of ops traits is not documented well,
but [this issue](https://github.com/verus-lang/verus/issues/1928) describes how to use them.

Cannot prove absence of panics, since the ops traits have to define it and they don't.

## Source

https://rosettacode.org/wiki/Ternary_logic

### License

GNU Free Document License 1.3
