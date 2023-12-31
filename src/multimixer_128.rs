use crunchy::unroll;

/// Each message block of Multimixer-128 is 32 -bytes wide.
pub const BLOCK_SIZE: usize = 32;
/// Message digest produced by Multimixer-128 is 64 -bytes wide.
pub const DIGEST_SIZE: usize = BLOCK_SIZE * 2;

// Number of u32 words, in input key/ message block (or chunk)
const IN_WORD_COUNT: usize = BLOCK_SIZE / std::mem::size_of::<u32>();
// Number of u64 words, in output digest block (or chunk)
const OUT_WORD_COUNT: usize = DIGEST_SIZE / std::mem::size_of::<u64>();

/// The public function of universal keyed hashing Multimixer, F-128
///
/// Given eight 32 -bit words as input, this routine applies F-128 and returns eight 64 -bit words.
/// This is an implementation of algorithm 2, in section 5 of paper https://ia.cr/2023/1357.
#[inline(always)]
pub const fn f_128(x: &[u32; IN_WORD_COUNT]) -> [u64; OUT_WORD_COUNT] {
    let mut u = [0u32; 4];
    let mut v = [0u32; 4];

    // Read definition 9 ( on page 11 ) of specification https://ia.cr/2023/1357,
    // which provides an alternative definition of F-128.
    //
    // u <- Nα · x
    // s.t. Nα <- circ(1, 1, 1, 0) <- [1, 1, 1, 0]
    //                                [0, 1, 1, 1]
    //                                [1, 0, 1, 1]
    //                                [1, 1, 0, 1]
    u[0] = x[0].wrapping_add(x[1]).wrapping_add(x[2]);
    u[1] = x[1].wrapping_add(x[2]).wrapping_add(x[3]);
    u[2] = x[2].wrapping_add(x[3]).wrapping_add(x[0]);
    u[3] = x[3].wrapping_add(x[0]).wrapping_add(x[1]);

    // v <- Nβ · y
    // s.t. Nβ <- circ(0, 1, 1, 1) <- [0, 1, 1, 1]
    //                                [1, 0, 1, 1]
    //                                [1, 1, 0, 1]
    //                                [1, 1, 1, 0]
    v[0] = x[5].wrapping_add(x[6]).wrapping_add(x[7]);
    v[1] = x[6].wrapping_add(x[7]).wrapping_add(x[4]);
    v[2] = x[7].wrapping_add(x[4]).wrapping_add(x[5]);
    v[3] = x[4].wrapping_add(x[5]).wrapping_add(x[6]);

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

/// Given 32 -bytes as input, this routine can be used for parsing input bytes as eight 32 -bit
/// little-endian unsigned integers.
#[inline(always)]
const fn get_data_block(data: &[u8; BLOCK_SIZE]) -> [u32; IN_WORD_COUNT] {
    const STEP: usize = std::mem::size_of::<u32>();

    let mut off = 0;
    let mut words = [0u32; 8];

    while off < data.len() {
        let word = ((data[off + 3] as u32) << 24)
            ^ ((data[off + 2] as u32) << 16)
            ^ ((data[off + 1] as u32) << 8)
            ^ (data[off + 0] as u32);

        words[off / STEP] = word;
        off += STEP;
    }

    words
}

/// Given two message blocks of length 32 -bytes, this routine adds them by performing word-wise
/// ( each word is of 32 -bits ) modulo addition ( modulo 2**32 ).
#[inline(always)]
const fn add_blocks(
    blk_a: &[u32; IN_WORD_COUNT],
    blk_b: &[u32; IN_WORD_COUNT],
) -> [u32; IN_WORD_COUNT] {
    let mut res = [0u32; 8];

    unroll! {
        for i in 0..8 {
            res[i] = blk_a[i].wrapping_add(blk_b[i]);
        }
    }
    res
}

/// Given 64 -bytes of input message block, this routine accumulates it into resulting digest of 64 -bytes,
/// by performing word-wise ( each word is of 64 -bits ) modulo addition ( modulo 2**64 ).
#[inline(always)]
const fn add_with_result(
    h: &[u64; OUT_WORD_COUNT],
    z: &[u64; OUT_WORD_COUNT],
) -> [u64; OUT_WORD_COUNT] {
    let mut h_prime = [0u64; 8];

    unroll! {
        for i in 0..8 {
            h_prime[i] = h[i].wrapping_add(z[i]);
        }
    }

    h_prime
}

/// Given n -bytes key and message s.t. n > 0 and n is a multiple of block size ( = 32 ), this routine
/// can be used for computing message digest of 64 -bytes, using universal keyed hashing algorithm based
/// on integer multiplication.
///
/// This is an implementation of algorithm 1, in section 3 of paper https://ia.cr/2023/1357.
#[inline(always)]
pub fn multimixer_128(key: &[u8], msg: &[u8], dig: &mut [u8; DIGEST_SIZE]) {
    debug_assert!(
        key.len() == msg.len(),
        "Both key and message must be of equal byte length !"
    );
    debug_assert!(msg.len() > 0, "Message must be non-empty !");
    debug_assert!(
        msg.len() % BLOCK_SIZE == 0,
        "Key/ Message byte length must be a multiple of block size i.e. 32 -bytes !"
    );

    let mut h = [0u64; 8]; // digest block, 64 -bytes

    let mut off = 0;
    while off < msg.len() {
        let key_blk = get_data_block(&key[off..(off + BLOCK_SIZE)].try_into().unwrap());
        let msg_blk = get_data_block(&msg[off..(off + BLOCK_SIZE)].try_into().unwrap());

        let x = add_blocks(&key_blk, &msg_blk);
        let z = f_128(&x);
        h = add_with_result(&h, &z);

        off += BLOCK_SIZE;
    }

    // Unpack digest words as little-endian bytes.
    unroll! {
        for i in 0..8 {
            let off = i * 8;
            dig[off..(off + 8)].copy_from_slice(&h[i].to_le_bytes());
        }
    }
}
