use flux_rs::assert;
use flux_rs::attrs::*;
use std::ops::Not;

#[refined_by(value: int)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Trit {
    #[variant(Trit[0])]
    True,
    #[variant(Trit[1])]
    Maybe,
    #[variant(Trit[2])]
    False,
}

impl Not for Trit {
    type Output = Self;

    #[sig(fn(self: Self) -> Self{t : (self.value == 0 => t.value == 2) && (self.value == 1 => t.value == 1) && (self.value == 2 => t.value == 0)})]
    fn not(self) -> Self {
        match self {
            Trit::True => Trit::False,
            Trit::Maybe => Trit::Maybe,
            Trit::False => Trit::True,
        }
    }
}
//
// impl BitAnd for Trit {
//     type Output = Self;
//
//     #[ensures(result == match (self, other) {
//         (Trit::True, Trit::True) => Trit::True,
//         (Trit::False, _) | (_, Trit::False) => Trit::False,
//         _ => Trit::Maybe,
//     })]
//     fn bitand(self, other: Self) -> Self {
//         match (self, other) {
//             (Trit::True, Trit::True) => Trit::True,
//             (Trit::False, _) | (_, Trit::False) => Trit::False,
//             _ => Trit::Maybe,
//         }
//     }
// }
//
// impl BitOr for Trit {
//     type Output = Self;
//
//     #[ensures(result == match (self, other) {
//         (Trit::True, _) | (_, Trit::True) => Trit::True,
//         (Trit::False, Trit::False) => Trit::False,
//         _ => Trit::Maybe,
//     })]
//     fn bitor(self, other: Self) -> Self {
//         match (self, other) {
//             (Trit::True, _) | (_, Trit::True) => Trit::True,
//             (Trit::False, Trit::False) => Trit::False,
//             _ => Trit::Maybe,
//         }
//     }
// }

impl Trit {
    // #[ensures((self == Trit::True ==> result == other)
    //     && (self == Trit::Maybe && other == Trit::True ==> result == Trit::True)
    //     && (self == Trit::Maybe && other != Trit::True ==> result == Trit::Maybe)
    //     && (self == Trit::False ==> result == Trit::True))]
    // fn imp(self, other: Self) -> Self {
    //     match self {
    //         Trit::True => other,
    //         Trit::Maybe => {
    //             if let Trit::True = other {
    //                 Trit::True
    //             } else {
    //                 Trit::Maybe
    //             }
    //         }
    //         Trit::False => Trit::True,
    //     }
    // }
    //
    // #[ensures((self == Trit::True ==> result == other)
    //     && (self == Trit::Maybe ==> result == Trit::Maybe)
    //     && (self == Trit::False ==> result == other.not_logic()))]
    // fn eqv(self, other: Self) -> Self {
    //     match self {
    //         Trit::True => other,
    //         Trit::Maybe => Trit::Maybe,
    //         Trit::False => other.not(),
    //     }
    // }
}

fn main() {
    let cn1 = Trit::True.not();
    assert(cn1 == Trit::False);
    let cn1 = Trit::Maybe.not();
    assert(cn1 == Trit::Maybe);

    // let ca1 = Trit::True & Trit::True;
    // assert(ca1 == Trit::True);
    // let ca2 = Trit::True & Trit::False;
    // assert(ca2 == Trit::False);
    // let ca3 = Trit::Maybe & Trit::True;
    // assert(ca3 == Trit::Maybe);
    //
    // let co1 = Trit::True | Trit::True;
    // assert(co1 == Trit::True);
    // let co2 = Trit::True | Trit::False;
    // assert(co2 == Trit::True);
    // let co3 = Trit::Maybe | Trit::False;
    // assert(co3 == Trit::Maybe);
    //
    // let ci1 = Trit::False.imp(Trit::True);
    // assert(ci1 == Trit::True);
    // let ci2 = Trit::True.imp(Trit::True);
    // assert(ci2 == Trit::True);
    // let ci3 = Trit::Maybe.imp(Trit::False);
    // assert(ci3 == Trit::Maybe);
    //
    // let ce1 = Trit::True.eqv(Trit::True);
    // assert(ce1 == Trit::True);
    // let ce2 = Trit::False.eqv(Trit::False);
    // assert(ce2 == Trit::True);
    // let ce3 = Trit::Maybe.eqv(Trit::False);
    // assert(ce3 == Trit::Maybe);
}
