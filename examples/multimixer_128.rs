use multimixer_128::{multimixer_128, BLOCK_SIZE, DIGEST_SIZE};
use rand::{thread_rng, RngCore};

fn main() {
    const BLOCKS: usize = 1;
    const MLEN: usize = BLOCK_SIZE * BLOCKS;

    let mut key = [0u8; MLEN];
    let mut msg = [0u8; MLEN];
    let mut dig = [0u8; DIGEST_SIZE];

    let mut rng = thread_rng();
    rng.fill_bytes(&mut key);
    rng.fill_bytes(&mut msg);

    multimixer_128(&key, &msg, &mut dig);

    println!("Key     = {}", hex::encode(&key));
    println!("Message = {}", hex::encode(&msg));
    println!("Digest  = {}", hex::encode(&dig));
}
