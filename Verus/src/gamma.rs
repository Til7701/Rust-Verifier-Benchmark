use vstd::prelude::*;

verus! {

fn binary_search(v: &Vec<u64>, k: u64) -> (r: usize)
    requires
        forall|i: int, j: int| 0 <= i <= j < v.len() ==> v[i] <= v[j],
        exists|i: int| 0 <= i < v.len() && k == v[i],
    ensures
        r < v.len(),
        k == v[r as int],
{
    let mut i1: usize = 0;
    let mut i2: usize = v.len() - 1;
    while i1 != i2
        invariant
            i2 < v.len(),
            exists|i: int| i1 <= i <= i2 && k == v[i],
            forall|i: int, j: int| 0 <= i <= j < v.len() ==> v[i] <= v[j],
        decreases i2 - i1,
    {
        let ix = i1 + (i2 - i1) / 2;
        if v[ix] < k {
            i1 = ix + 1;
        } else {
            i2 = ix;
        }
    }
    i1
}

fn main() {
    let mut v: Vec<u64> = Vec::new();
    v.push(0);
    v.push(10);
    v.push(20);
    v.push(30);
    v.push(40);
    assert(v[3] == 30);  // needed to trigger exists|i: int| ... k == v[i]
    let r = binary_search(&v, 30);
    assert(r == 3);
}
use vstd::prelude::*;

verus! {

fn gnome_sort(v: &mut Vec<i32>)
    ensures
        sorted(v@),
        v@.to_multiset() == old(v)@.to_multiset(),
{
    let mut i: usize = 0;

    while i < v.len()
        invariant
            i <= v.len(),

            // permutation preserved
            v@.to_multiset() == old(v)@.to_multiset(),

            // prefix before i is sorted
            sorted_between(v@, 0, i as int),

            // if i is not at either end, the only possible
            // disorder is at the boundary (i-1,i)
            forall|j: int, k: int|
                0 <= j < k < i ==> v@[j] <= v@[k],
    {
        if i == 0 || v[i - 1] <= v[i] {
            i += 1;
        } else {
        let tmp = v[i - 1];
        v[i - 1] = v[i];
        v[i] = tmp;

            proof {
                // show multiset unchanged by swap
                assert(v@.to_multiset() =~=
                       old(v)@.to_multiset());
            }

            i -= 1;
        }
    }

    assert(i == v.len());
}

spec fn sorted(s: Seq<i32>) -> bool {
    forall|i: int, j: int|
        0 <= i < j < s.len() ==> s[i] <= s[j]
}

spec fn sorted_between(s: Seq<i32>, lo: int, hi: int) -> bool {
    forall|i: int, j: int|
        lo <= i < j < hi ==> s[i] <= s[j]
}

}

// fn concatenate_arrays<T: Clone>(x: &[T], y: &[T]) -> (result: Vec<T>)
//     requires
//         x.len() + y.len() <= usize::MAX,
//     ensures
//         result@.len() == x@.len() + y@.len(),
//         result@ =~= x@ + y@,
// {
//     let mut concat = Vec::with_capacity(x.len() + y.len());
//
//     for i in x.iter() {
//         concat.push(i.clone());
//     }
//
//     for j in y.iter() {
//         concat.push(j.clone());
//     }
//
//     concat
// }
//
// fn main() {
//     let l1 = vec![1, 2, 3, 4, 5];
//     let l2 = vec![6, 7, 8, 9, 10];
//
//     let c = concatenate_arrays(&l1, &l2);
//     assert(c@ =~= seq![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
// }
} // verus!
