pub fn assert(b: bool) {
    if !b {
        panic!("assertion failed");
    }
}

fn main() {
    assert(true);
}
