#![cfg(test)]

use crate::multimixer_128::{multimixer_128, BLOCK_SIZE, DIGEST_SIZE};
use std::fs::File;
use std::io::{BufRead, BufReader};

#[cfg(debug_assertions)]
#[test]
#[should_panic = "Message must be non-empty !"]
fn test_empty_key_and_msg() {
    let mut dig = [0u8; DIGEST_SIZE];
    multimixer_128(&[], &[], &mut dig);
}

#[cfg(debug_assertions)]
#[test]
#[should_panic = "Both key and message must be of equal byte length !"]
fn test_empty_key_and_nonempty_msg() {
    let msg = [0u8; BLOCK_SIZE];
    let mut dig = [0u8; DIGEST_SIZE];
    multimixer_128(&[], &msg, &mut dig);
}

#[cfg(debug_assertions)]
#[test]
#[should_panic = "Key/ Message byte length must be a multiple of block size i.e. 32 -bytes !"]
fn test_unsupported_msg_len() {
    const MIN_MSG_LEN: usize = 1;
    const MAX_MSG_LEN: usize = BLOCK_SIZE * 2;

    let mut dig = [0u8; DIGEST_SIZE];
    let mut mlen = MIN_MSG_LEN;

    while mlen <= MAX_MSG_LEN {
        let key = vec![0x0fu8; mlen];
        let msg = vec![0xf0u8; mlen];

        multimixer_128(&key, &msg, &mut dig);

        mlen = if (mlen + 1) % BLOCK_SIZE == 0 {
            mlen + 2
        } else {
            mlen + 1
        };
    }
}

#[test]
fn test_known_answer_tests() {
    const FILE: &str = "./multimixer_128.kat";

    let fd = File::open(FILE).unwrap();
    let mut lines = BufReader::new(fd).lines();

    while let Some(line) = lines.next() {
        let mlen = line.unwrap().split(" = ").collect::<Vec<&str>>()[1]
            .parse::<usize>()
            .unwrap();
        let key = hex::decode(
            lines
                .next()
                .unwrap()
                .unwrap()
                .split(" = ")
                .collect::<Vec<&str>>()[1],
        )
        .unwrap();
        let msg = hex::decode(
            lines
                .next()
                .unwrap()
                .unwrap()
                .split(" = ")
                .collect::<Vec<&str>>()[1],
        )
        .unwrap();
        let md = hex::decode(
            lines
                .next()
                .unwrap()
                .unwrap()
                .split(" = ")
                .collect::<Vec<&str>>()[1],
        )
        .unwrap();

        assert!(mlen > 0);
        assert!(mlen % BLOCK_SIZE == 0);
        let mut computed_md = [0u8; DIGEST_SIZE];

        multimixer_128(&key, &msg, &mut computed_md);
        assert_eq!(md, computed_md);

        lines.next(); // empty line, at end of each test case
    }
}
