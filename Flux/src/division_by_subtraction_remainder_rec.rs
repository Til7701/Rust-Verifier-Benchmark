use flux_rs::assert;
use flux_rs::attrs::*;

#[refined_by(q: int, r: int)]
struct QR {
    #[field(u64[q])]
    q: u64,
    #[field(u64[r])]
    r: u64,
}

// #[assoc(
//     fn is_eq(x: Self, y: Self, res: bool) -> bool { res <=> (x == y) }
//     fn is_ne(x: Self, y: Self, res: bool) -> bool { res <=> (x == y) }
// )]
// impl PartialEq for QR {
//     #[spec(fn(&Self[@s], &Self[@t]) -> bool{v: Self::is_eq(s, t, v)})]
//     fn eq(&self, other: &Self) -> bool {
//         self.n == other.n && self.d == other.d && self.q == other.q && self.r == other.r
//     }
//
//     #[spec(fn(&Self[@s], &Self[@t]) -> bool{v: Self::is_ne(s, t, v)})]
//     fn ne(&self, other: &Self) -> bool {
//         self.n != other.n || self.d != other.d || self.q != other.q || self.r != other.r
//     }
// }

#[spec(fn(n: u64, d: u64{d >= 1}) -> QR{qr: qr.q == n / d && qr.r == n % d})]
fn remainder_rec(n: u64, d: u64) -> QR {
    if n < d {
        QR { q: 0, r: n }
    } else {
        let qr = remainder_rec(n - d, d);
        QR {
            q: qr.q + 1,
            r: qr.r,
        }
    }
}

fn main() {
    let res = remainder_rec(10, 3);
    assert(res.q == 3);
    assert(res.r == 1);
    // let qr = QR {
    //     q: 3,
    //     r: 1,
    // };
    //assert(qr == res);
}
