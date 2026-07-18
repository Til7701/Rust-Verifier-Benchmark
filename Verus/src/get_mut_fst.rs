use vstd::prelude::*;

verus! {

fn get_mut_fst<A, B>(pair: &mut (A, B)) -> (ret: &mut A)
    ensures
        *ret == old(pair).0,
        *final(pair) == (*final(ret), old(pair).1),
{
    &mut pair.0
}

fn main() {
    let mut pair = (10, 20);
    let r = get_mut_fst(&mut pair);
    *r = 100;
    assert(pair == (100, 20));
}

} // verus!
