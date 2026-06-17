use flux_rs::{no_panic, should_fail, spec};

#[no_panic]
#[spec(fn(b:bool[true]))]
pub fn assert(b: bool) {
    if !b {
        panic!("assertion failed");
    }
}

fn main() {
    assert(true);
}

#[should_fail]
fn fails() {
    assert(false);
}
