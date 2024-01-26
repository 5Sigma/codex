use core::Project;
use std::sync::{Arc, RwLock};

use console::style;
use human_repr::{HumanCount, HumanDuration};
use rayon::prelude::{IntoParallelIterator, ParallelBridge, ParallelIterator};
use tiny_http::{Request, Response, Server};

use crate::RootCommands;

struct ServerHandler {
    project: Arc<RwLock<Project>>,
}

pub fn serve(args: &crate::Args) {
    let RootCommands::Serve { port } = &args.command else {
        panic!("Expected Serve command");
    };
    let server_url = format!("0.0.0.0:{}", port);
    let term = console::Term::stdout();
    let _ = term.write_line(&format!(
        "{} {}",
        style("Starting server on").green().dim(),
        style(&server_url).bright().underlined().bold()
    ));
    let server = Server::http(server_url).unwrap();
    let handler = ServerHandler {
        project: Arc::new(RwLock::new(
            Project::load(&args.root_path, true).expect("Failed to load project"),
        )),
    };

    server
        .incoming_requests()
        .par_bridge()
        .into_par_iter()
        .for_each(|request| {
            let now = std::time::Instant::now();
            let url = request.url().to_string();
            if url == "/" {
                let _ = handler.project.write().unwrap().reload();
                output_log(&url, now.elapsed(), handler.handle_file(request));
            } else if handler
                .project
                .read()
                .unwrap()
                .path
                .from_url(&format!("/static{}", request.url()))
                .exists()
            {
                let size = handler.handle_static(request);
                if args.verbose {
                    output_log(&url, now.elapsed(), size);
                }
            } else {
                let _ = handler.project.write().unwrap().reload();
                let size = handler.handle_file(request);
                output_log(&url, now.elapsed(), size);
            }
        });
}

impl ServerHandler {
    #[allow(dead_code)]
    pub fn handle_static(&self, request: Request) -> usize {
        let static_path = self
            .project
            .read()
            .unwrap()
            .path
            .from_url(&format!("/static{}", request.url()));
        if let Ok(data) = static_path.read() {
            let len = data.len();
            let _ = request.respond(Response::from_data(data));
            len
        } else {
            respond404(&self.project.read().unwrap(), request)
        }
    }
    pub fn handle_file(&self, request: Request) -> usize {
        let url = if request.url() == "/" {
            request.url()
        } else {
            request.url().trim_end_matches('/')
        };
        if let Some(doc) = self.project.read().unwrap().get_document_for_url(url) {
            let page_content = match doc.page_content(&self.project.read().unwrap()) {
                Ok(i) => i,
                Err(e) => format!("<pre>{}</pre>", e),
            };
            let l = page_content.len();
            let response = Response::from_string(page_content).with_header(tiny_http::Header {
                field: "Content-Type".parse().unwrap(),
                value: "text/html".parse().unwrap(),
            });
            let _ = request.respond(response);
            l
        } else {
            respond404(&self.project.read().unwrap(), request)
        }
    }
}

pub fn respond404(project: &Project, request: Request) -> usize {
    let ctx = core::DataContext {
        project: project.details.clone(),
        ..Default::default()
    };
    let data = core::render_template(ctx, &core::assets::get_str("_internal/templates/404.html"))
        .unwrap_or_default();
    let _ = request.respond(
        tiny_http::Response::from_string(data)
            .with_header(tiny_http::Header {
                field: "Content-Type".parse().unwrap(),
                value: "text/html".parse().unwrap(),
            })
            .with_status_code(404),
    );
    0
}
fn output_log(url: &str, time: std::time::Duration, size: usize) {
    if size == 0 {
        return;
    }
    let term = console::Term::stdout();
    let stats = format!("{} in {}", size.human_count_bytes(), time.human_duration());
    let _ = term.write_line(&format!("{:>20} {}", style(stats).dim(), style(url).bold()));
}
