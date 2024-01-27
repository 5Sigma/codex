use core::{Project, Renderer};
use std::path::PathBuf;

use criterion::{criterion_group, criterion_main, Criterion};

fn criterion_benchmark(c: &mut Criterion) {
    let project = Project::load(PathBuf::from("test").join("fixture"), false).unwrap();
    let doc = project.get_document_for_url("/pages/large").unwrap();

    let renderer = core::HtmlRenderer {
        render_context: core::RenderContext {
            base_url: project.details.base_url.clone(),
            file_path: doc.file_path.clone(),
            root_folder: project.root_folder.clone(),
            root_path: project.path.clone(),
            front_matter: doc.frontmatter.clone(),
            project_details: project.details.clone(),
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
