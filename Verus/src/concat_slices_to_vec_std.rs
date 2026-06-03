use vstd::prelude::*;

verus! {

fn concat_slices_to_vec_std<T: Clone>(x: &[T], y: &[T]) -> (result: Vec<T>)
    requires
        x.len() + y.len() <= usize::MAX,
    ensures
        result@.len() == x@.len() + y@.len(),
        forall|i: int| 0 <= i < x.len() ==> cloned::<T>(#[trigger] x@[i], result@[i]),
        forall|i: int| 0 <= i < y.len() ==> cloned::<T>(#[trigger] y@[i], result@[x.len() + i]),
{
    let mut concat = Vec::with_capacity(x.len() + y.len());
    concat.extend_from_slice(x);
    concat.extend_from_slice(y);
    concat
}

fn main() {
    let l1 = vec![1, 2, 3, 4, 5];
    let l2 = vec![6, 7, 8, 9, 10];

    let c = concat_slices_to_vec_std(&l1, &l2);
    assert(c@ =~= seq![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
}

} // verus!
