use vstd::prelude::*;

verus! {

fn concat_slices_to_vec_mix<T: Copy>(x: &[T], y: &[T]) -> (result: Vec<T>)
    requires
        x.len() + y.len() <= usize::MAX,
    ensures
        result@.len() == x@.len() + y@.len(),
        forall|i: int| 0 <= i < x.len() ==> cloned::<T>(#[trigger] x@[i], result@[i]),
        result@.skip(x.len() as int) =~= y@,
{
    let mut concat = Vec::with_capacity(x.len() + y.len());
    concat.extend_from_slice(&x);

    for e in iter: y
        invariant
            concat.len() == x.len() + iter.index(),
            forall|j: int| 0 <= j < x.len() ==> cloned::<T>(#[trigger] x@[j], concat@[j]),
            concat@.skip(x.len() as int) =~= y@.take(iter.index()),
    {
        concat.push(*e);
    }
    concat
}

} // verus!
