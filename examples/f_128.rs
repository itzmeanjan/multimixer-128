use multimixer_128::f_128;
use static_assertions::const_assert;

const fn eq_slice<const L: usize>(a: &[u64; L], b: &[u64; L]) -> bool {
    let mut acc = false;
    let mut idx = 0;

    while idx < L {
        acc |= (a[idx] ^ b[idx]) != 0;
        idx += 1;
    }

    !acc
}

const fn main() {
    const X: [u32; 8] = [0xff00ff00; 8];
    const Y: [u64; 8] = f_128(&X);
    const EXPECTED_Y: [u64; 8] = [
        0xfe02fc02fe010000,
        0xfe02fc02fe010000,
        0xfe02fc02fe010000,
        0xfe02fc02fe010000,
        0xfa0ee81aee090000,
        0xfa0ee81aee090000,
        0xfa0ee81aee090000,
        0xfa0ee81aee090000,
    ];

    // Must hold f_128(x) == y !
    const_assert!(eq_slice(&Y, &EXPECTED_Y));
}
