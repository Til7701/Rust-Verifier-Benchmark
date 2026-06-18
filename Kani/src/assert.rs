pub fn assert(b: bool) {
    if !b {
        panic!("assertion failed");
    }
}

#[cfg(kani)]
mod verification {
    use super::*;

    #[kani::proof]
    fn main() {
        assert(true);
    }

    #[kani::proof]
    #[kani::should_panic]
    fn fails() {
        assert(false);
    }
}
