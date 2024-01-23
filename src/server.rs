use core::Project;
use std::path::{Path, PathBuf};

use tiny_http::{Request, Response, Server};

struct ServerHandler {
    project: Project,
}

pub fn serve(project_path: String) {
    let server_url = "0.0.0.0:8080";
    let term = console::Term::stdout();
    let _ = term.write_line(&format!(
        "Starting server on {}",
        console::Style::new()
            .white()
            .underlined()
            .bright()
            .bold()
            .apply_to(server_url)
    ));
    let server = Server::http(server_url).unwrap();
    let mut handler = ServerHandler {
        project: Project::load(project_path, true).unwrap(),
    };

    for request in server.incoming_requests() {
        let now = std::time::Instant::now();
        let _ = handler.project.reload();
        let url = request.url().to_string();
        if url == "/" {
            handler.handle_file(request);
        } else if core::assets::exists(
            &PathBuf::from("static")
                .join(request.url().trim_matches('/'))
                .display()
                .to_string(),
        ) || convert_static_url_to_path(&handler.project.path, &url).exists()
        {
            handler.handle_static(request);
            // output_log(&url, now.elapsed());
        } else {
            handler.handle_file(request);
            output_log(&url, now.elapsed());
        }
    }
}

impl ServerHandler {
    #[allow(dead_code)]
    pub fn handle_static(&self, request: Request) {
        let path = &PathBuf::from("static").join(request.url().trim_matches('/'));
        if self.project.path.join(path).exists() {
            if let Ok(f) = std::fs::read(self.project.path.join(path)) {
                let response = Response::from_data(f);
                let _ = request.respond(response);
            } else {
                let _ = request.respond(Response::from_string("Failed to read data"));
            }
        } else {
            let f = core::assets::get_bytes(&path.display().to_string());
            let response = Response::from_data(f);
            let _ = request.respond(response);
        }
    }
    pub fn handle_file(&self, request: Request) {
        let url = if request.url() == "/" {
            request.url()
        } else {
            request.url().trim_end_matches('/')
        };
        if let Some(doc) = self.project.get_document_for_url(url) {
            let response = Response::from_string(doc.page_content(&self.project).unwrap())
                .with_header(tiny_http::Header {
                    field: "Content-Type".parse().unwrap(),
                    value: "text/html".parse().unwrap(),
                });
            let _ = request.respond(response);
        } else {
            let ctx = core::DataContext {
                project: self.project.details.clone(),
                ..Default::default()
            };
            let data = core::render_template(ctx, &core::assets::get_str("templates/404.html"))
                .unwrap_or_default();
            let _ = request.respond(
                tiny_http::Response::from_string(data)
                    .with_header(tiny_http::Header {
                        field: "Content-Type".parse().unwrap(),
                        value: "text/html".parse().unwrap(),
                    })
                    .with_status_code(404),
            );
        }
    }
}

fn output_log(url: &str, time: std::time::Duration) {
    let term = console::Term::stdout();
    let dim = console::Style::new().dim();
    let bold = console::Style::new().bold().bright();
    let time = dim.apply_to(format!("{}ms", time.as_millis()));
    let url = bold.apply_to(url);
    let _ = term.write_line(&format!("[{}] {}", time, url));
}

fn convert_static_url_to_path(root: &Path, url: &str) -> PathBuf {
    url.split('/')
        .fold(root.join("static"), |acc, t| acc.join(t))
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use crate::server::convert_static_url_to_path;

    #[test]
    fn test_convert_static_url() {
        assert_eq!(
            convert_static_url_to_path(&PathBuf::from("/"), "/subpath/something.css")
                .display()
                .to_string(),
            PathBuf::from("/static")
                .join("subpath")
                .join("something.css")
                .display()
                .to_string(),
        );
    }
}
