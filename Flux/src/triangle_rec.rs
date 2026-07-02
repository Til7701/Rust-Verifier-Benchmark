use flux_rs::assert;
use flux_rs::attrs::*;

#[no_panic]
#[spec(fn(n: u64) -> u64{result: result == (n * (n + 1)) / 2})]
fn triangle(n: u64) -> u64 {
    if n == 0 { 0 } else { n + triangle(n - 1) }
}

fn main() {
    assert(triangle(5) == 15);
}
