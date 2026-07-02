use std::ops::{BitAnd, BitOr, Not};
use vstd::prelude::*;
use vstd::std_specs::ops::*;

verus! {

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Trit {
    True,
    Maybe,
    False,
}

impl Not for Trit {
    type Output = Self;

    fn not(self) -> (result: Self)
        ensures
            self is True ==> result is False,
            self is Maybe ==> result is Maybe,
            self is False ==> result is True,
    {
        match self {
            Trit::True => Trit::False,
            Trit::Maybe => Trit::Maybe,
            Trit::False => Trit::True,
        }
    }
}

impl NotSpecImpl for Trit {
    open spec fn obeys_not_spec() -> bool {
        true
    }

    open spec fn not_req(self) -> bool {
        true
    }

    open spec fn not_spec(self) -> Self::Output {
        match self {
            Trit::True => Trit::False,
            Trit::Maybe => Trit::Maybe,
            Trit::False => Trit::True,
        }
    }
}

impl BitAnd for Trit {
    type Output = Self;

    fn bitand(self, other: Self) -> (result: Self)
        ensures
            (self is True && other is True ==> result is True) || (self is False || other is False
                ==> result is False) || (result is Maybe),
    {
        match (self, other) {
            (Trit::True, Trit::True) => Trit::True,
            (Trit::False, _) | (_, Trit::False) => Trit::False,
            _ => Trit::Maybe,
        }
    }
}

impl BitAndSpecImpl for Trit {
    open spec fn obeys_bitand_spec() -> bool {
        true
    }

    open spec fn bitand_req(self, rhs: Self) -> bool {
        true
    }

    open spec fn bitand_spec(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Trit::True, Trit::True) => Trit::True,
            (Trit::False, _) | (_, Trit::False) => Trit::False,
            _ => Trit::Maybe,
        }
    }
}

impl BitOr for Trit {
    type Output = Self;

    fn bitor(self, other: Self) -> (result: Self)
        ensures
            (self is True || other is True ==> result is True) || (self is False && other is False
                ==> result is False) || (result is Maybe),
    {
        match (self, other) {
            (Trit::True, _) | (_, Trit::True) => Trit::True,
            (Trit::False, Trit::False) => Trit::False,
            _ => Trit::Maybe,
        }
    }
}

impl BitOrSpecImpl for Trit {
    open spec fn obeys_bitor_spec() -> bool {
        true
    }

    open spec fn bitor_req(self, rhs: Self) -> bool {
        true
    }

    open spec fn bitor_spec(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Trit::True, _) | (_, Trit::True) => Trit::True,
            (Trit::False, Trit::False) => Trit::False,
            _ => Trit::Maybe,
        }
    }
}

impl Trit {
    fn imp(self, other: Self) -> (result: Self)
        ensures
            self is True ==> result == other,
            self is Maybe && other is True ==> result is True,
            //self is Maybe && other !is True ==> result is Maybe, // Causes RustRover to mark everything as an error
            self is Maybe && !(other is True) ==> result is Maybe,
            self is False ==> result is True,
        no_unwind
    {
        match self {
            Trit::True => other,
            Trit::Maybe => {
                if let Trit::True = other {
                    Trit::True
                } else {
                    Trit::Maybe
                }
            },
            Trit::False => Trit::True,
        }
    }

    fn eqv(self, other: Self) -> (result: Self)
        ensures
            self is True ==> result == other,
            self is Maybe ==> result is Maybe,
            self is False ==> result == NotSpec::not_spec(other),
    {
        match self {
            Trit::True => other,
            Trit::Maybe => Trit::Maybe,
            Trit::False => other.not(),
        }
    }
}

fn main() {
    let cn1 = Trit::True.not();
    assert(cn1 == Trit::False);
    let cn1 = Trit::Maybe.not();
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

} // verus!
