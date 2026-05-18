// use creusot_std::prelude::*;

// #[check(terminates)]
// #[variant(m)]
// // #[ensures(m@ == 0 ==> result == n + 1u64)]
// // #[ensures(m@ > 0 && n@ == 0 ==> result == ack(m - 1u64, 1u64))]
// // #[ensures(m@ > 0 && n@ > 0 ==> result == ack(m - 1u64, ack(m, n - 1u64)))]
// fn ack(m: u64, n: u64) -> u64 {
//     if m == 0 {
//         n + 1
//     } else if n == 0 {
//         ack(m - 1, 1)
//     } else {
//         ack(m - 1, ack(m, n - 1))
//     }
// }
