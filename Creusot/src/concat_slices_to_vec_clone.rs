use creusot_std::prelude::*;

#[requires(x@.len() + y@.len() <= usize::MAX@)]
#[ensures(result@.len() == x@.len() + y@.len())]
#[ensures(forall<i> 0 <= i && i < x@.len() ==> T::clone.postcondition((&x@[i],), result@[i]))]
#[ensures(forall<i> 0 <= i && i < y@.len() ==> T::clone.postcondition((&y@[i],), result@[x@.len() + i]))]
fn concat_slices_to_vec_clone<T: Clone>(x: &[T], y: &[T]) -> Vec<T> {
    let mut concat: Vec<T> = Vec::with_capacity(x.len() + y.len());

    #[invariant(concat@.len() == (*produced).len())]
    #[invariant(forall<i> 0 <= i && i < (*produced).len() ==> T::clone.postcondition((&x@[i],), concat@[i]))]
    for e in x.iter() {
        concat.push(e.clone());
    }

    #[invariant(concat@.len() == x@.len() + (*produced).len())]
    #[invariant(forall<i> 0 <= i && i < x@.len() ==> T::clone.postcondition((&x@[i],), concat@[i]))]
    #[invariant(forall<i> 0 <= i && i < (*produced).len() ==> T::clone.postcondition((&y@[i],), concat@[x@.len() + i]))]
    for e in y.iter() {
        concat.push(e.clone());
    }
    concat
}

fn main() {
    let l1 = creusot_std::prelude::vec![1, 2, 3, 4, 5];
    let l2 = creusot_std::prelude::vec![6, 7, 8, 9, 10];

    let c = concat_slices_to_vec_clone(&l1, &l2);
    proof_assert!(c@ == seq![1i32, 2i32, 3i32, 4i32, 5i32, 6i32, 7i32, 8i32, 9i32, 10i32]);
}
