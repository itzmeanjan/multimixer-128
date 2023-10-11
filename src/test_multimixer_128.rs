#![cfg(test)]

use crate::multimixer_128::{multimixer_128, BLOCK_SIZE, DIGEST_SIZE};
use std::fs::File;
use std::io::{BufRead, BufReader};

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
