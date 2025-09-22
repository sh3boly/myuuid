use std::hint::black_box;
use criterion::{criterion_group, criterion_main, Criterion};
use uuid_rust::MyUuid;

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("new_v7", |b| {
        b.iter(|| {
            // Call static method directly
            let uuid = MyUuid::new_v7();
            black_box(uuid) // prevent optimizer from throwing it away
        })
    });

    c.bench_function("new_v4", |b| {
        b.iter(|| {
            let uuid = MyUuid::new_v4();
            black_box(uuid)
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
