use flux_rs::assert;
use flux_rs::attrs::*;

#[spec(fn (x: &mut i32[@n]) ensures x: i32[n + 1])]
pub fn incr(x: &mut i32) {
    *x += 1;
}

fn main() {
    let mut p = 10;
    incr(&mut p);
    assert(p == 11);
}
