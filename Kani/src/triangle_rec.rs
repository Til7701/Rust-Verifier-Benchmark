#[cfg(kani)]
mod verification {
    use super::*;

    #[kani::requires(((n as u64 * (n as u64 + 1)) / 2) <= u16::MAX as u64)]
    #[kani::ensures(|result| *result as u64 == ((n as u64 * (n as u64 + 1)) / 2))]
    #[kani::recursion]
    fn triangle_rec(n: u16) -> u16 {
        if n == 0 { 0 } else { n + triangle_rec(n - 1) }
    }

    #[kani::proof]
    fn main() {
        let r = triangle_rec(10);
        assert_eq!(r, 55);
    }

    #[kani::proof_for_contract(triangle_rec)]
    fn check_triangle_rec() {
        let n: u16 = kani::any();
        triangle_rec(n);
    }

    #[kani::proof]
    #[kani::stub_verified(triangle_rec)]
    fn verify_triangle_rec() {
        let n: u16 = kani::any_where(|n| (*n as u64 * (*n as u64 + 1)) / 2 <= u16::MAX as u64);
        let result = triangle_rec(n);
        assert_eq!(result, ((n as u64 * (n as u64 + 1)) / 2) as u16);
    }
}
