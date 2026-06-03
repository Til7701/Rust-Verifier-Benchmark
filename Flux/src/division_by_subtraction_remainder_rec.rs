// use flux_rs::assert;
// use flux_rs::attrs::*;
//
// #[refined_by(n: int, d: int, q: int, r: int)]
// #[invariant(d >= 1)]
// #[invariant(d > r)]
// #[invariant(n == d * q + r)]
// #[invariant(q == n / d)]
// #[invariant(r == n % d)]
// #[derive(PartialEq)]
// struct QR {
//     #[field(u64[n])]
//     n: u64,
//     #[field(u64[d])]
//     d: u64,
//     #[field(u64[q])]
//     q: u64,
//     #[field(u64[r])]
//     r: u64,
// }
//
// #[spec(fn(n: u64, d: u64{d >= 1}) -> QR)]
// fn remainder_rec(n: u64, d: u64) -> QR {
//     if n < d {
//         QR { n, d, q: 0, r: n }
//     } else {
//         let qr = remainder_rec(n - d, d);
//         let q = qr.q + 1;
//         let r = qr.r;
//         assert(d * q + r == n + ((n - d) * q + r));
//         QR { n, d, q, r }
//     }
// }
//
// fn main() {
//     assert(
//         remainder_rec(10, 3)
//             == QR {
//                 n: 10,
//                 d: 3,
//                 q: 3,
//                 r: 1,
//             },
//     );
// }
