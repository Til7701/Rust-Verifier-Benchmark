use creusot_std::prelude::*;
use std::ops::{BitAnd, BitOr, Not};

#[derive(Copy, creusot_std::prelude::Clone, creusot_std::prelude::PartialEq, Eq)]
pub enum Trit {
    True,
    Maybe,
    False,
}

impl DeepModel for Trit {
    type DeepModelTy = u8;

    #[logic]
    fn deep_model(self) -> Self::DeepModelTy {
        match self {
            Trit::True => 0u8,
            Trit::Maybe => 1u8,
            Trit::False => 2u8,
        }
    }
}

impl Not for Trit {
    type Output = Self;

    #[ensures((self == Trit::True ==> result == Trit::False)
        && (self == Trit::Maybe ==> result == Trit::Maybe)
        && (self == Trit::False ==> result == Trit::True))]
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

    #[ensures(result == match (self, other) {
        (Trit::True, Trit::True) => Trit::True,
        (Trit::False, _) | (_, Trit::False) => Trit::False,
        _ => Trit::Maybe,
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

    #[ensures(result == match (self, other) {
        (Trit::True, _) | (_, Trit::True) => Trit::True,
        (Trit::False, Trit::False) => Trit::False,
        _ => Trit::Maybe,
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
    #[logic]
    fn not_logic(self) -> Self {
        match self {
            Trit::True => Trit::False,
            Trit::Maybe => Trit::Maybe,
            Trit::False => Trit::True,
        }
    }

    #[ensures((self == Trit::True ==> result == other)
        && (self == Trit::Maybe && other == Trit::True ==> result == Trit::True)
        && (self == Trit::Maybe && other != Trit::True ==> result == Trit::Maybe)
        && (self == Trit::False ==> result == Trit::True))]
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

    #[ensures((self == Trit::True ==> result == other)
        && (self == Trit::Maybe ==> result == Trit::Maybe)
        && (self == Trit::False ==> result == other.not_logic()))]
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
    proof_assert!(cn1 == Trit::False);
    let cn1 = !Trit::Maybe;
    proof_assert!(cn1 == Trit::Maybe);

    let ca1 = Trit::True & Trit::True;
    proof_assert!(ca1 == Trit::True);
    let ca2 = Trit::True & Trit::False;
    proof_assert!(ca2 == Trit::False);
    let ca3 = Trit::Maybe & Trit::True;
    proof_assert!(ca3 == Trit::Maybe);

    let co1 = Trit::True | Trit::True;
    proof_assert!(co1 == Trit::True);
    let co2 = Trit::True | Trit::False;
    proof_assert!(co2 == Trit::True);
    let co3 = Trit::Maybe | Trit::False;
    proof_assert!(co3 == Trit::Maybe);

    let ci1 = Trit::False.imp(Trit::True);
    proof_assert!(ci1 == Trit::True);
    let ci2 = Trit::True.imp(Trit::True);
    proof_assert!(ci2 == Trit::True);
    let ci3 = Trit::Maybe.imp(Trit::False);
    proof_assert!(ci3 == Trit::Maybe);

    let ce1 = Trit::True.eqv(Trit::True);
    proof_assert!(ce1 == Trit::True);
    let ce2 = Trit::False.eqv(Trit::False);
    proof_assert!(ce2 == Trit::True);
    let ce3 = Trit::Maybe.eqv(Trit::False);
    proof_assert!(ce3 == Trit::Maybe);
}
