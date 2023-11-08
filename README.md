# multimixer-128
Multimixer-128: Universal Keyed Hashing Based on Integer Multiplication

## Overview

$Multimixer_{128}$ is a universal keyed hashing algorithm, based on just integer arithmetic ( addition and multiplication ), proposed in paper https://ia.cr/2023/1357. 

A keyed hash function, takes a key and a variable length message as input, compressing it to a fixed length digest. The keyed hashing scheme proposed in $Multimixer_{128}$ paper, takes equal length key and message ( meaning long keys will be required, in case message is long ) s.t. they are multiple of block size ( = 32 -bytes ) and compresses them into a 64 -bytes digest. It's proved to be an ε-∆universal hash function with ε = $2^{−127}$. It can be used during the compression phase of a message authentication code computation scheme. It can also be used in the compression phase of the Farfalle construction - which are used for building deck functions.

As $Multimixer_{128}$ is just based on integer arithmetic, which is pretty fast on most processors, it shows quite promising results during benchmarks. See [below](#benchmarking).

You can also find more details on how to start using $Multimixer_{128}$ API, [below](#usage).

## Prerequisites

Rust stable toolchain; see https://rustup.rs for installation guide.

```bash
# When developing this library, I was using
$ rustc --version
rustc 1.73.0 (cc66ad468 2023-10-03)
```

I advise you to use `cargo-criterion` for running benchmark executable. Read more about it @ https://crates.io/crates/cargo-criterion. You can just issue following command for installing it system-wide.

```bash
cargo install cargo-criterion
```

## Testing

For ensuring functional correctness and conformance to the specification of $Multimixer_{128}$ i.e. https://ia.cr/2023/1357 ( and its reference implementation https://github.com/Parisaa/Multimixer ), I use known answer tests that I've generated by following instructions described in https://gist.github.com/itzmeanjan/a32eab0244af55eba2847c6472337535.

```bash
cargo test --lib
```

## Benchmarking

Issue following command for benchmarking public function $f_{128}$ and $Multimixer_{128}$, with variable ( non-zero multiple of 32 -bytes ) length input key and message, on target CPU.

> **Note**
When benchmarking on `x86`, `x86_64`, `loongarch64` targets, CPU cycles and cycles/ byte metrics are reported, using `rdtsc` instruction, though for other targets, default wallclock timer of criterion.rs is used for reporting time and throughput. I found https://github.com/pornin/crrl/blob/73b33c1efc73d637f3084d197353991a22c10366/benches/util.rs pretty useful for obtaining CPU cycles when benchmarking Rust functions. But I'm using criterion.rs as benchmark harness, hence I decided to go with https://crates.io/crates/criterion-cycles-per-byte plugin, much easier to integrate.

> **Warning**
When benchmarking make sure you've disabled CPU frequency scaling, otherwise numbers you see can be pretty misleading. I found https://github.com/google/benchmark/blob/b40db869/docs/reducing_variance.md helpful.

```bash
# In case you didn't install `cargo-criterion`, you have to run benchmark with
# RUSTFLAGS="-C opt-level=3 -C target-cpu=native" cargo bench --features="internal"

RUSTFLAGS="-C opt-level=3 -C target-cpu=native" cargo criterion --features="internal"
```

### On *12th Gen Intel(R) Core(TM) i7-1260P*

```bash
f_128/f-128 (cached)    time:   [6.9927 cycles 7.0443 cycles 7.1069 cycles]                      
                        thrpt:  [0.2221 cpb 0.2201 cpb 0.2185 cpb]
f_128/f-128 (random)    time:   [21.1852 cycles 21.3297 cycles 21.4551 cycles]                   
                        thrpt:  [0.6705 cpb 0.6666 cpb 0.6620 cpb]

multimixer_128/32B key/ msg (cached)                                                                             
                        time:   [14.0913 cycles 14.1296 cycles 14.1664 cycles]
                        thrpt:  [0.2214 cpb 0.2208 cpb 0.2202 cpb]
multimixer_128/32B key/ msg (random)                                                                            
                        time:   [63.4513 cycles 63.8913 cycles 64.3942 cycles]
                        thrpt:  [1.0062 cpb 0.9983 cpb 0.9914 cpb]

multimixer_128/128B key/ msg (cached)                                                                             
                        time:   [42.2552 cycles 42.2975 cycles 42.3421 cycles]
                        thrpt:  [0.1654 cpb 0.1652 cpb 0.1651 cpb]
multimixer_128/128B key/ msg (random)                                                                            
                        time:   [107.6240 cycles 109.1370 cycles 111.0124 cycles]
                        thrpt:  [0.4336 cpb 0.4263 cpb 0.4204 cpb]

multimixer_128/512B key/ msg (cached)                                                                            
                        time:   [166.6061 cycles 166.8388 cycles 167.0822 cycles]
                        thrpt:  [0.1632 cpb 0.1629 cpb 0.1627 cpb]
multimixer_128/512B key/ msg (random)                                                                            
                        time:   [239.1786 cycles 240.4487 cycles 241.9385 cycles]
                        thrpt:  [0.2363 cpb 0.2348 cpb 0.2336 cpb]

multimixer_128/2048B key/ msg (cached)                                                                            
                        time:   [606.0156 cycles 606.3316 cycles 606.6701 cycles]
                        thrpt:  [0.1481 cpb 0.1480 cpb 0.1480 cpb]
multimixer_128/2048B key/ msg (random)                                                                             
                        time:   [835.5582 cycles 848.0583 cycles 859.5641 cycles]
                        thrpt:  [0.2099 cpb 0.2070 cpb 0.2040 cpb]

multimixer_128/8192B key/ msg (cached)                                                                             
                        time:   [2346.9930 cycles 2347.5554 cycles 2348.2118 cycles]
                        thrpt:  [0.1433 cpb 0.1433 cpb 0.1432 cpb]
multimixer_128/8192B key/ msg (random)                                                                             
                        time:   [2816.3210 cycles 2838.9367 cycles 2859.1557 cycles]
                        thrpt:  [0.1745 cpb 0.1733 cpb 0.1719 cpb]
```

## Usage

Using $Multimixer_{128}$ hasher API is pretty straight-forward.

1) Add `multimixer-128` to the *[dependencies]* section of the Cargo.toml file of your project.

```toml
[dependencies]
multimixer-128 = { git = "https://github.com/itzmeanjan/multimixer-128" }
# or
multimixer-128 = "0.1.0"

# In case you're also interested in using f_128, the public function of multimixer-128,
# enable `internal` feature-gate of this crate.
#
# multimixer-128 = { version = "0.1.0", features = "internal" }
```

2) Get non-zero, equal length key and message s.t. their byte length is a multiple of block size (= 32 -bytes), as input.

```rust
use multimixer_128;

fn main() {
    const BLOCKS: usize = 1;
    const MLEN: usize = multimixer_128::BLOCK_SIZE * BLOCKS;

    // Beware, this is just an example key !
    let key = [0x0fu8; MLEN];
    let msg = [0xf0u8; MLEN];
    let mut dig = [0u8; multimixer_128::DIGEST_SIZE];

    // ...
}
```

3) Compute 64 -bytes message digest, given equal length (non-zero) key and message.

```rust
fn main() {
    // ...

    multimixer_128::multimixer_128(&key, &msg, &mut dig);
    assert_eq!(dig, [1, 0, 0, 0, 254, 255, 255, 255, 1, 0, 0, 0, 254, 255, 255, 255, 1, 0, 0, 0, 254, 255, 255, 255, 1, 0, 0, 0, 254, 255, 255, 255, 9, 0, 0, 0, 250, 255, 255, 255, 9, 0, 0, 0, 250, 255, 255, 255, 9, 0, 0, 0, 250, 255, 255, 255, 9, 0, 0, 0, 250, 255, 255, 255]);
}
```

I'm maintaining an example program, demonstrating usage of $Multimixer_{128}$ API, living inside [examples](./examples/) directory.

```bash
cargo run --example multimixer_128

Key     = e02f28d290bd51df3b103396debe7f2ffc90b837588e37b13a7cc8cccce9fa11
Message = 1511ad54daae3df6706e33d9953f5dff4eabb2da85ad1added1aba2e0b571397
Digest  = d2324595d842cb028205c1f00328ba760d292a690452726d95728070ee7ff21e8e6af14b1e4c6d15404e709fd1fec952da3e3b7b3fe7d038fca793f3b4d7661a
```

> **Note**
Most of the internal functions of this library crate are implemented as `const fn` i.e. compile-time evaluable functions. That's why I also maintain another example [f_128.rs](./examples/f_128.rs) which demonstrates that property of this library using static assertions. Try executing the example program by issuing `$ cargo run --example f_128 --features="internal"` and you'll see it.
