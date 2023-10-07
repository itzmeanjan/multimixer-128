use criterion::{black_box, criterion_group, criterion_main, BatchSize, Criterion, Throughput};
use rand::{thread_rng, Rng};

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

criterion_group!(multimixer_128, f_128);
criterion_main!(multimixer_128);
