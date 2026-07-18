fn get_mut_fst<A, B>(pair: &mut (A, B)) -> &mut A {
    &mut pair.0
}

#[cfg(kani)]
mod verification {
    use super::*;

    #[kani::proof]
    fn main() {
        let mut pair = (10, 20);
        let r = get_mut_fst(&mut pair);
        *r = 100;
        assert_eq!(pair, (100, 20));
    }
}
