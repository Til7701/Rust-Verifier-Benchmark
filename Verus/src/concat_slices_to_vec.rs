use vstd::prelude::*;

verus! {

fn concat_slices_to_vec<T: Copy>(x: &[T], y: &[T]) -> (result: Vec<T>)
    requires
        x.len() + y.len() <= usize::MAX,
    ensures
        result@.len() == x@.len() + y@.len(),
        result@ =~= x@ + y@,
{
    let mut concat = Vec::with_capacity(x.len() + y.len());

    for i in 0..x.len()
        invariant
            concat.len() == i,
            concat@ =~= x@.subrange(0, i as int),
    {
        concat.push(x[i]);
    }

    for i in 0..y.len()
        invariant
            concat.len() == x.len() + i,
            concat@ =~= x@ + y@.subrange(0, i as int),
    {
        concat.push(y[i]);
    }

    concat
}

fn main() {
    let l1 = vec![1, 2, 3, 4, 5];
    let l2 = vec![6, 7, 8, 9, 10];

    let c = concat_slices_to_vec(&l1, &l2);
    assert(c@ =~= seq![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
}

} // verus!
