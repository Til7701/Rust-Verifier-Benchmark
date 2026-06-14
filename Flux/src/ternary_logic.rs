use flux_rs::assert;
use flux_rs::attrs::*;
use std::ops::{BitAnd, BitOr, Not};

defs!(
    fn trit_not(t1: Trit, t2: Trit) -> bool {
        (t1 == Trit::True => t2 == Trit::False)
        && (t1 == Trit::Maybe => t2 == Trit::Maybe)
        && (t1 == Trit::False => t2 == Trit::True)
    }
);

#[reflect]
#[derive(Copy, Clone)]
pub enum Trit {
    True,
    Maybe,
    False,
}

impl PartialEq for Trit {
    #[no_panic]
    #[spec(fn(&Self[@r1], &Self[@r2]) -> bool[r1 == r2])]
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Trit::True, Trit::True) => true,
            (Trit::Maybe, Trit::Maybe) => true,
            (Trit::False, Trit::False) => true,
            _ => false,
        }
    }
}

impl Not for Trit {
    type Output = Self;

    #[no_panic]
    #[sig(fn(self: Self) -> Self{result : trit_not(self, result)})]
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

    #[no_panic]
    #[sig(fn(self: Self, other: Self) -> Self{result :
        if (self == Trit::True && other == Trit::True) { result == Trit::True } else
        if (self == Trit::False || other == Trit::False) { result == Trit::False } else
        { result == Trit::Maybe }
    })]
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

    #[no_panic]
    #[sig(fn(self: Self, other: Self) -> Self{result :
        if (self == Trit::True || other == Trit::True) { result == Trit::True } else
        if (self == Trit::False && other == Trit::False) { result == Trit::False } else
        { result == Trit::Maybe }
    })]
    fn bitor(self, other: Self) -> Self {
        match (self, other) {
            (Trit::True, _) | (_, Trit::True) => Trit::True,
            (Trit::False, Trit::False) => Trit::False,
            _ => Trit::Maybe,
        }
    }
}

impl Trit {
    #[no_panic]
    #[sig(fn(self: Self, other: Self) -> Self{result :
        if self == Trit::True { result == other } else
        if (self == Trit::Maybe && other == Trit::True) { result == Trit::True } else
        if (self == Trit::Maybe && other != Trit::True) { result == Trit::Maybe } else
        { result == Trit::True }
    })]
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

    #[no_panic]
    #[sig(fn(self: Self, other: Self) -> Self{result :
        if self == Trit::True { result == other } else
        if self == Trit::Maybe { result == Trit::Maybe } else
        { trit_not(result, other) }
    })]
    fn eqv(self, other: Self) -> Self {
        match self {
            Trit::True => other,
            Trit::Maybe => Trit::Maybe,
            Trit::False => other.not(),
        }
    }
}

fn main() {
    let cn1 = !Trit::True;
    assert(cn1 == Trit::False);
    let cn1 = !Trit::Maybe;
    assert(cn1 == Trit::Maybe);

    let ca1 = Trit::True & Trit::True;
    assert(ca1 == Trit::True);
    let ca2 = Trit::True & Trit::False;
    assert(ca2 == Trit::False);
    let ca3 = Trit::Maybe & Trit::True;
    assert(ca3 == Trit::Maybe);

    let co1 = Trit::True | Trit::True;
    assert(co1 == Trit::True);
    let co2 = Trit::True | Trit::False;
    assert(co2 == Trit::True);
    let co3 = Trit::Maybe | Trit::False;
    assert(co3 == Trit::Maybe);

    let ci1 = Trit::False.imp(Trit::True);
    assert(ci1 == Trit::True);
    let ci2 = Trit::True.imp(Trit::True);
    assert(ci2 == Trit::True);
    let ci3 = Trit::Maybe.imp(Trit::False);
    assert(ci3 == Trit::Maybe);

    let ce1 = Trit::True.eqv(Trit::True);
    assert(ce1 == Trit::True);
    let ce2 = Trit::False.eqv(Trit::False);
    assert(ce2 == Trit::True);
    let ce3 = Trit::Maybe.eqv(Trit::False);
    assert(ce3 == Trit::Maybe);
}
