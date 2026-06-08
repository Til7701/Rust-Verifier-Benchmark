use flux_rs::assert;
use flux_rs::attrs::*;

#[spec(fn(n: u64) -> bool{result: (n % 2 == 0) <=> result})]
fn is_even(n: u64) -> bool {
    if n == 0 {
        return true;
    }
    is_odd(n - 1)
}

#[spec(fn(n: u64) -> bool{result: (n % 2 == 1) <=> result})]
fn is_odd(n: u64) -> bool {
    if n == 0 {
        return false;
    }
    is_even(n - 1)
}

fn main() {
    let e1 = is_even(5);
    assert(!e1);
    let e2 = is_even(42);
    assert(e2);

    let o1 = is_odd(5);
    assert(o1);
    let o2 = is_odd(42);
    assert(!o2);
}
