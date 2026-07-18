fn get_mut_fst<A, B>(pair: &mut (A, B)) -> &mut A {
    &mut pair.0
}

fn main() -> (i32, i32) {
    let mut pair = (10, 20);
    let r = get_mut_fst(&mut pair);
    *r = 100;
    pair
}
