use core::Project;
use std::path::PathBuf;

use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn criterion_benchmark(c: &mut Criterion) {
    let project = Project::load(PathBuf::from("test").join("fixture"), false).unwrap();
    let doc = project.get_document_for_url("/pages/large").unwrap();

    c.bench_function("document-body-render", |b| b.iter(|| doc.body()));
    c.bench_function("document-full-page-render", |b| {
        b.iter(|| doc.page_content(black_box(&project)))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
