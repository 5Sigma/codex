use core::Project;
use std::path::PathBuf;

use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("project-load", |b| {
        b.iter(|| Project::load(black_box(PathBuf::from("test").join("fixture")), false).unwrap())
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
