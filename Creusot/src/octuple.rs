use creusot_std::prelude::*;

#[check(terminates)]
#[requires(-16i8 <= x1)]
#[requires(x1 < 16i8)]
#[ensures(result == x1 * 8i8)]
fn octuple(x1: i8) -> i8 {
    let x2 = x1 + x1;
    let x4 = x2 + x2;
    x4 + x4
}
