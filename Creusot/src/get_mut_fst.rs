use creusot_std::prelude::*;

#[ensures(result == &mut pair.0)]
#[ensures(^pair == (^result, pair.1))]
fn get_mut_fst<A, B>(pair: &mut (A, B)) -> &mut A {
    &mut pair.0
}

fn main() {
    let mut pair = (10, 20);
    let r = get_mut_fst(&mut pair);
    *r = 100;
    proof_assert!(pair == (100i32, 20i32));
}
