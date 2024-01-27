use core::{Project, Renderer};
use std::path::PathBuf;

use criterion::{criterion_group, criterion_main, Criterion};

fn criterion_benchmark(c: &mut Criterion) {
    let project = Project::load(PathBuf::from("test").join("fixture"), false).unwrap();
    let doc = project.get_document_for_url("/pages/large").unwrap();

    let renderer = core::HtmlRenderer {
        render_context: core::RenderContext {
            project: &project,
            document: doc,
        },
    };

    c.bench_function("document-body-render", |b| {
        b.iter(|| renderer.render_body())
    });
    c.bench_function("document-full-page-render", |b| {
        b.iter(|| renderer.render())
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
