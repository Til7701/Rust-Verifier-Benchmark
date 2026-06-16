use vstd::prelude::*;

verus! {

#[verifier::exec_allows_no_decreases_clause]
fn gnome_sort(v: &mut Vec<i32>)
    ensures
        final(v)@.len() == old(v)@.len(),
        forall|i: int, j: int| 0 <= i <= j < final(v)@.len() ==> final(v)@[i] <= final(v)@[j],
        final(v)@.to_multiset() =~= old(v)@.to_multiset(),
{
    let ghost old_v: Seq<i32> = v@;
    let mut i = 0;
    while i < v.len()
        invariant
            forall|j: int, k: int| 0 <= j <= k < i ==> v[j] <= v[k],
            v@.to_multiset() =~= old_v.to_multiset(),
            v@.len() == old_v.len(),
    {
        if i == 0 {
            i += 1;
        } else if v[i - 1] <= v[i] {
            i += 1;
        } else {
            v.swap(i - 1, i);
            i -= 1;
        }
    }
}

pub assume_specification<T>[ <[T]>::swap ](s: &mut [T], a: usize, b: usize)
    requires
        a < s@.len(),
        b < s@.len(),
    ensures
        final(s)@[a as int] == old(s)@[b as int],
        final(s)@[b as int] == old(s)@[a as int],
        final(s)@.to_multiset() =~= old(s)@.to_multiset(),
        final(s)@.len() == old(s)@.len(),
        forall|i: int|
            (0 <= i < final(s)@.len() && i != a && i != b) ==> old(s)@[i] == final(s)@[i],
;

// fn main() {
// assert(seq![0i32, 1i32].to_multiset() =~= seq![1i32, 0i32].to_multiset());
//     let mut v = vec![10, 0, 543, 24, 31, 30];
//     gnome_sort(&mut v);
//     assert(v@ =~= seq![0, 10, 24, 30, 31, 543]);
//
//     proof {
//         use vstd::seq_lib::group_seq_properties;
//         broadcast use group_seq_properties;
//
//         let ghost result: Seq<i32> = seq![0, 10, 10, 24, 30, 31, 543];
//         assert(seq![0i32, 10i32, 10i32, 24i32, 30i32, 31i32, 543i32].to_multiset() =~= seq![
//             10i32,
//             0i32,
//             543i32,
//             24i32,
//             31i32,
//             30i32,
//             10i32,
//         ].to_multiset());
//         assert(v@.to_multiset() == result.to_multiset());
//         vstd::seq_lib::lemma_sorted_unique(result, v@, |a: i32, b: i32| a <= b);
//         assert(v@.len() == result.len());
//         assert(v@.contains(0));
//         assert(v@ =~= result);
//     }
//
// }
} // verus!
