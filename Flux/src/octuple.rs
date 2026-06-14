use flux_rs::attrs::*;

#[no_panic]
#[spec(fn(x: i8) -> i8{v: v == 8 * x})]
fn octuple(x1: i8) -> i8 {
    let x2 = x1 + x1;
    let x4 = x2 + x2;
    x4 + x4
}
