use criterion::Benchmark;
use criterion::Criterion;
use criterion::Throughput;
use std::time::Duration;

const SIZE: usize = 1024 * 1024;

fn large_setup(c: &mut Criterion) {
    c.bench(
        "iter_with_large_setup",
        Benchmark::new("large_setup", |b| {
            b.iter_with_large_setup(
                || (0..SIZE).map(|i| i as u8).collect::<Vec<_>>(),
                |v| v.clone(),
            )
        })
        .throughput(Throughput::Bytes(SIZE as u32)),
    );
}

fn small_setup(c: &mut Criterion) {
    c.bench(
        "iter_with_large_setup",
        Benchmark::new("small_setup", |b| {
            b.iter_with_large_setup(|| SIZE, |size| size)
        }),
    );
}

fn short_warmup() -> Criterion {
    Criterion::default().warm_up_time(Duration::new(1, 0))
}

criterion_group! {
    name = benches;
    config = short_warmup();
    targets = large_setup, small_setup
}
