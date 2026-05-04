use prusti_contracts::*;

#[requires(-16 <= x1)]
#[requires(x1 < 16)]
#[ensures(result == x1 * 8)]
fn octuple(x1: i8) -> i8 {
    let x2 = x1 + x1;
    let x4 = x2 + x2;
    x4 + x4
}
