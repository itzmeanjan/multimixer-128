use criterion::{black_box, criterion_group, criterion_main, BatchSize, Criterion, Throughput};
use rand::{thread_rng, Rng, RngCore};

fn f_128(c: &mut Criterion) {
    let mut rng = thread_rng();

    let mut group = c.benchmark_group("f_128");
    group.throughput(Throughput::Bytes(multimixer_128::BLOCK_SIZE as u64)); // size of f-128 input block, in bytes

    group.bench_function("f-128 (cached)", |bench| {
        let mut x = [0u32; 8];
        rng.fill(&mut x);

        bench.iter(|| black_box(multimixer_128::f_128(black_box(&x))))
    });
    group.bench_function("f-128 (random)", |bench| {
        let mut x = [0u32; 8];
        rng.fill(&mut x);

        bench.iter_batched(
            || x.clone(),
            |mut x| black_box(multimixer_128::f_128(black_box(&mut x))),
            BatchSize::SmallInput,
        )
    });

    group.finish();
}

fn multimixer_128<const ILEN: usize>(c: &mut Criterion) {
    assert!((ILEN > 0) && (ILEN % multimixer_128::BLOCK_SIZE == 0));

    let mut rng = thread_rng();

    let mut group = c.benchmark_group("multimixer_128");
    group.throughput(Throughput::Bytes((2 * ILEN) as u64));

    group.bench_function(format!("/{} (cached)", ILEN), |bench| {
        let mut key = vec![0u8; ILEN];
        let mut msg = vec![0u8; ILEN];
        let mut dig = [0u8; multimixer_128::DIGEST_SIZE];

        rng.fill_bytes(&mut key);
        rng.fill_bytes(&mut msg);

        bench.iter(|| {
            multimixer_128::multimixer_128(black_box(&key), black_box(&msg), black_box(&mut dig))
        })
    });
    group.bench_function(format!("/{} (random)", ILEN), |bench| {
        let mut key = vec![0u8; ILEN];
        let mut msg = vec![0u8; ILEN];
        let mut dig = [0u8; multimixer_128::DIGEST_SIZE];

        rng.fill_bytes(&mut key);
        rng.fill_bytes(&mut msg);

        bench.iter_batched(
            || (key.clone(), msg.clone()),
            |(_key, _msg)| {
                multimixer_128::multimixer_128(
                    black_box(&_key),
                    black_box(&_msg),
                    black_box(&mut dig),
                )
            },
            BatchSize::SmallInput,
        )
    });

    group.finish();
}

criterion_group!(
    keyed_hashing,
    f_128,
    multimixer_128<32>,
    multimixer_128<64>,
    multimixer_128<128>,
    multimixer_128<256>,
    multimixer_128<512>,
    multimixer_128<1024>,
    multimixer_128<2048>,
    multimixer_128<4096>
);
criterion_main!(keyed_hashing);
