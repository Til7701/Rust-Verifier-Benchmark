fn triangle_rec(n: u64) -> u64 {
    match n {
        0 => 0,
        _ => n + triangle_rec(n - 1),
    }
}

fn main() {
    let r = triangle_rec(10);
    assert_eq!(r, 55);
}

// Unwinds forever
// #[cfg(kani)]
// #[kani::proof]
// fn verify_triangle_rec() {
//     let n: u64 = kani::any_where(|n| (n * (n + 1)) / 2 <= u64::MAX);
//     let result = triangle_rec(n);
//     assert_eq!(result, (n * (n + 1)) / 2);
// }
