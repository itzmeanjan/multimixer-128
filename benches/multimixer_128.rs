use criterion::{black_box, criterion_group, criterion_main, BatchSize, Criterion, Throughput};
use rand::{thread_rng, Rng, RngCore};
#[cfg(any(
    target_arch = "x86_64",
    target_arch = "x86",
    target_arch = "loongarch64"
))]
use criterion_cycles_per_byte::CyclesPerByte;

#[cfg(any(
    target_arch = "x86_64",
    target_arch = "x86",
    target_arch = "loongarch64"
))]
type CriterionCPB = Criterion<CyclesPerByte>;

#[cfg(not(any(
    target_arch = "x86_64",
    target_arch = "x86",
    target_arch = "loongarch64"
)))]
type CriterionCPB = Criterion;

fn f_128(c: &mut CriterionCPB) {
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

fn multimixer_128(c: &mut CriterionCPB) {
    let mut rng = thread_rng();

    const MIN_MLEN: usize = multimixer_128::BLOCK_SIZE;
    const MAX_MLEN: usize = 4096;

    let mut mlen = MIN_MLEN;
    while mlen <= MAX_MLEN {
        assert!((mlen > 0) && (mlen % multimixer_128::BLOCK_SIZE == 0));

        let mut group = c.benchmark_group("multimixer_128");
        group.throughput(Throughput::Bytes((2 * mlen) as u64));

        group.bench_function(format!("{} (cached)", mlen), |bench| {
            let mut key = vec![0u8; mlen];
            let mut msg = vec![0u8; mlen];
            let mut dig = [0u8; multimixer_128::DIGEST_SIZE];

            rng.fill_bytes(&mut key);
            rng.fill_bytes(&mut msg);

            bench.iter(|| {
                multimixer_128::multimixer_128(
                    black_box(&key),
                    black_box(&msg),
                    black_box(&mut dig),
                )
            })
        });
        group.bench_function(format!("{} (random)", mlen), |bench| {
            let mut key = vec![0u8; mlen];
            let mut msg = vec![0u8; mlen];
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
        mlen = 2 * mlen;
    }
}

#[cfg(any(
    target_arch = "x86_64",
    target_arch = "x86",
    target_arch = "loongarch64"
))]
criterion_group!(name = keyed_hashing; config = Criterion::default().with_measurement(CyclesPerByte); targets = f_128, multimixer_128);

#[cfg(not(any(
    target_arch = "x86_64",
    target_arch = "x86",
    target_arch = "loongarch64"
)))]
criterion_group!(keyed_hashing, f_128, multimixer_128);

criterion_main!(keyed_hashing);
