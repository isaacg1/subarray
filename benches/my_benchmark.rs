use criterion::{black_box, criterion_group, criterion_main, Criterion};
use subarray::solve;

pub fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("My Group");
    group.sample_size(10);
    group.bench_function("solve 12", |b| b.iter(|| solve(black_box(12))));
    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
