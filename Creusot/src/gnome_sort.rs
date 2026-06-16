use creusot_std::prelude::*;

#[ensures((^v).deep_model().sorted())]
#[ensures((^v)@.permutation_of(v@))]
fn gnome_sort<T: Ord + DeepModel>(v: &mut [T])
where
    T::DeepModelTy: OrdLogic,
{
    let _old_v = snapshot! { v };
    let mut i = 0;
    #[invariant(v.deep_model().sorted_range(0, i@))]
    #[invariant(v@.permutation_of(_old_v@))]
    while i < v.len() {
        if i == 0 || v[i - 1] <= v[i] {
            i += 1;
        } else {
            v.swap(i - 1, i);
            i -= 1;
        }
    }
}

#[test]
fn main() {
    let mut v = creusot_std::prelude::vec![10, 0, 543, 24, 31, 30, 10];
    gnome_sort(&mut v);
    proof_assert!(v@ == seq![0i32, 10i32, 10i32, 24i32, 30i32, 31i32, 543i32]);
}
