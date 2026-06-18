use vstd::prelude::*;

verus! {

fn concat_slices_to_vec_clone<T: Clone>(x: &[T], y: &[T]) -> (result: Vec<T>)
    requires
        x.len() + y.len() <= usize::MAX,
    ensures
        result@.len() == x@.len() + y@.len(),
        forall|i: int| 0 <= i < x.len() ==> cloned::<T>(#[trigger] x@[i], result@[i]),
        forall|i: int| 0 <= i < y.len() ==> cloned::<T>(#[trigger] y@[i], result@[x.len() + i]),
{
    let mut concat = Vec::with_capacity(x.len() + y.len());

    for i in 0..x.len()
        invariant
            concat.len() == i,
            forall|j: int| 0 <= j < i ==> cloned::<T>(#[trigger] x@[j], concat@[j]),
    {
        concat.push(x[i].clone());
    }

    for i in 0..y.len()
        invariant
            concat.len() == x.len() + i,
            forall|j: int| 0 <= j < x.len() ==> cloned::<T>(#[trigger] x@[j], concat@[j]),
            forall|j: int| 0 <= j < i ==> cloned::<T>(#[trigger] y@[j], concat@[x.len() + j]),
    {
        concat.push(y[i].clone());
    }

    concat
}

fn main() {
    let l1 = vec![1, 2, 3, 4, 5];
    let l2 = vec![6, 7, 8, 9, 10];

    let c = concat_slices_to_vec_clone(&l1, &l2);
    assert(c@ =~= seq![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
}

} // verus!
