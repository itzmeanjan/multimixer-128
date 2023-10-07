/// The public function of universal keyed hashing Multimixer, F-128
///
/// Given eight 32 -bit words as input, this routine applies F-128 and returns eight 64 -bit words.
#[inline(always)]
fn f_128(x: &[u32; 8]) -> [u64; 8] {
    let mut u = [0u32; 4];
    let mut v = [0u32; 4];

    u[0] = x[0].wrapping_add(x[1]).wrapping_add(x[2]);
    u[1] = x[1].wrapping_add(x[2]).wrapping_add(x[3]);
    u[2] = x[2].wrapping_add(x[3]).wrapping_add(x[0]);
    u[3] = x[3].wrapping_add(x[0]).wrapping_add(x[1]);

    v[0] = x[4].wrapping_add(x[5]).wrapping_add(x[6]);
    v[1] = x[5].wrapping_add(x[6]).wrapping_add(x[7]);
    v[2] = x[6].wrapping_add(x[7]).wrapping_add(x[4]);
    v[3] = x[7].wrapping_add(x[4]).wrapping_add(x[5]);

    let mut z = [0u64; 8];

    z[0] = (x[0] as u64) * (x[4] as u64);
    z[1] = (x[1] as u64) * (x[5] as u64);
    z[2] = (x[2] as u64) * (x[6] as u64);
    z[3] = (x[3] as u64) * (x[7] as u64);

    z[4] = (u[0] as u64) * (v[0] as u64);
    z[5] = (u[1] as u64) * (v[1] as u64);
    z[6] = (u[2] as u64) * (v[2] as u64);
    z[7] = (u[3] as u64) * (v[3] as u64);

    z
}
