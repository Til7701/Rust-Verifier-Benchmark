fn octuple(x1: i8) -> i8 {
    let x2 = x1 + x1;
    let x4 = x2 + x2;
    x4 + x4
}

#[cfg(kani)]
#[kani::proof]
fn verify_octuple() {
    let x1: i8 = kani::any();
    kani::assume(-16 <= x1 && x1 < 16);
    let x8 = octuple(x1);
    assert!(x8 == x1 * 8)
}
