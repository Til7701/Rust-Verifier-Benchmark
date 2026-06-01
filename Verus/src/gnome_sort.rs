verus! {

fn gnome_sort(v: &mut Vec<i32>)
    ensures
        sorted(v@),
        v@.to_multiset() == old(v)@.to_multiset(),
{
    let mut i = 0;
    while i < v.len()
        invariant
            i <= v.len(),
            v@.to_multiset() == old(v)@.to_multiset(),
            sorted_between(v@, 0, i as int),
        decreases inversions(v@), v.len() - i,
    {
        if i == 0 || v[i - 1] <= v[i] {
            i += 1;
        } else {
            let ghost inv = inversions(v@);
            let tmp = v[i - 1];
            v[i - 1] = v[i];
            v[i] = tmp;

            assert(inversions(v@) < inv);
            assert(v@.to_multiset() =~= old(v)@.to_multiset());

            i -= 1;
        }
    }

    assert(i == v.len());
}

spec fn sorted(s: Seq<i32>) -> bool {
    forall|i: int, j: int| 0 <= i < j < s.len() ==> s[i] <= s[j]
}

spec fn sorted_between(s: Seq<i32>, min: int, max: int) -> bool {
    forall|i: int, j: int| min <= i < j < max ==> s[i] <= s[j]
}

spec fn inversions(s: Seq<i32>) -> nat
    decreases s.len(),
{
    if s.len() <= 1 {
        0
    } else {
        let head = s[0];
        let tail = s.drop_first();

        (count_greater(head as int, tail) as nat) + inversions(tail)
    }
}

spec fn count_greater(x: int, s: Seq<i32>) -> nat
    decreases s.len(),
{
    if s.len() == 0 {
        0
    } else {
        (if x > s[0] {
            1nat
        } else {
            0nat
        }) + count_greater(x, s.drop_first())
    }
}

} // verus!

use vstd::prelude::*;
