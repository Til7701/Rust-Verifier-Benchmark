fn ackermann(m: u64, n: u64) -> u64 {
    if m == 0 {
        n + 1
    } else if n == 0 {
        ackermann(m - 1, 1)
    } else {
        ackermann(m - 1, ackermann(m, n - 1))
    }
}

#[cfg(kani)]
mod verification {
    use super::*;

    #[kani::proof]
    fn main() {
        let a = ackermann(2, 4);
        assert!(a == 11);
    }
}
