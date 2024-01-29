use std::collections::HashMap;

use crate::{error::Result, render_template, DataContext, Error};
use handlebars::html_escape;
use markdown::mdast::Node;

use super::{RenderContext, Renderer};

pub struct HtmlRenderer<'a> {
    pub render_context: RenderContext<'a>,
}

impl<'a> HtmlRenderer<'a> {
    pub fn new(render_context: RenderContext<'a>) -> Self {
        Self { render_context }
    }
    /// Wrap a list of nodes in HTML with the provided start and end fragments
    fn wrap_nodes(&self, start: &str, end: &str, nodes: &[Node]) -> Result<String> {
        Ok(format!("{}{}{}", start, self.render_nodes(nodes)?, end))
    }

    fn apply_component(
        &self,
        name: &str,
        mut attrs: HashMap<String, String>,
        children: &[Node],
    ) -> Result<String> {
        let cmp_path = self
            .render_context
            .document
            .file_path
            .new_path(format!("_internal/components/{}.html", name.to_lowercase()));

        if cmp_path.exists() {
            attrs.insert("children".to_string(), self.render_nodes(children)?);
            render_template(attrs, &String::from_utf8(cmp_path.read()?.to_vec())?)
        } else {
            Ok("<pre>Unknown Component</pre>".to_string())
        }
    }
}

impl Renderer for HtmlRenderer<'_> {
    fn get_context(&self) -> &RenderContext {
        &self.render_context
    }
    fn finalize_render(&self, data: DataContext) -> Result<String> {
        crate::render_template(
            data,
            &String::from_utf8(
                self.render_context
                    .project
                    .path
                    .new_path("_internal/templates/article.html")
                    .read()?
                    .to_vec(),
            )?,
        )
    }
    fn render_blockquote(&self, children: &[Node]) -> Result<String> {
        self.wrap_nodes(r#"<p class="lead">"#, "</p>", children)
    }

    fn render_jsx_element(
        &self,
        name: &str,
        attrs: std::collections::HashMap<String, String>,
        children: &[Node],
    ) -> Result<String> {
        self.apply_component(name, attrs, children)
    }

    fn render_list(&self, ordered: bool, children: &[Node]) -> Result<String> {
        if ordered {
            self.wrap_nodes("<ol>", "</ol>", children)
        } else {
            self.wrap_nodes("<ul>", "</ul>", children)
        }
    }

    fn render_inline_code(&self, code: &str) -> Result<String> {
        Ok(format!(r#"<code class="inline">{}</code>"#, code))
    }

    fn render_delete(&self, children: &[Node]) -> Result<String> {
        self.wrap_nodes(
            "<span style=\"text-decoration: line-through\">",
            "</span>",
            children,
        )
    }
    fn render_emphasis(&self, children: &[Node]) -> Result<String> {
        self.wrap_nodes(r#"<span class="fst-italic">"#, "</span>", children)
    }

    fn render_image(&self, url: &str, title: Option<String>) -> Result<String> {
        Ok(format!(
            r#"<img class="img-fluid" src="{}" title="{}""#,
            url,
            title.unwrap_or_default()
        ))
    }

    fn render_link(&self, url: &str, title: Option<String>, children: &[Node]) -> Result<String> {
        Ok(format!(
            "<a href=\"{}\" alt=\"{}\">{}</a>",
            url,
            title.unwrap_or_default(),
            self.render_nodes(children)?
        ))
    }

    fn render_bold(&self, children: &[Node]) -> Result<String> {
        self.wrap_nodes(r#"<span class="fw-bold">"#, "</span>", children)
    }

    fn render_code(
        &self,
        code: &str,
        lang: Option<String>,
        filepath: Option<std::path::PathBuf>,
    ) -> Result<String> {
        let lines = if let Some(ref lang) = lang {
            highlight(lang, code.trim()).unwrap_or(
                html_escape(code)
                    .lines()
                    .map(|s| s.to_string())
                    .collect::<Vec<_>>(),
            )
        } else if let Some(ref filepath) = filepath {
            highlight_by_extension(filepath, code.trim()).unwrap_or(
                html_escape(code)
                    .lines()
                    .map(|s| s.to_string())
                    .collect::<Vec<_>>(),
            )
        } else {
            html_escape(code)
                .lines()
                .map(|s| s.to_string())
                .collect::<Vec<_>>()
        };
        let template = self
            .render_context
            .document
            .file_path
            .new_path("_internal/templates/code.html");
        let data = super::CodeContext {
            lines,
            lang: lang.clone().unwrap_or_default(),
        };
        template
            .read()
            .map(|d| d.to_vec())
            .map_err(crate::Error::from)
            .and_then(|d| {
                render_template(data, &String::from_utf8(d).unwrap_or_default())
                    .map_err(crate::Error::from)
            })
    }

    fn render_heading(&self, depth: u8, children: &[Node]) -> Result<String> {
        let Some(text) = self.get_text(children) else {
            return Ok("<pre>No header text found</pre>".to_string());
        };
        let slug = self.slug(&text);
        let tag = format!("h{}", depth + 3);
        let html = children.iter().try_fold(String::new(), |acc, child| {
            self.render_node(child).map(|s| format!("{}{}", acc, s))
        })?;
        Ok(format!(
            "<{} class=\"mt-4\" id=\"{}\">{}</{}>",
            tag, slug, html, tag
        ))
    }

    fn render_thematic_break(&self) -> Result<String> {
        Ok("<hr/>".to_string())
    }
    fn render_table_row(&self, children: &[Node]) -> Result<String> {
        self.wrap_nodes("<tr>", "</tr>", children)
    }

    fn render_table(&self, content: String) -> Result<String> {
        Ok(format!(
            r#"<table class="table table-sm table-striped">{}</table>"#,
            content
        ))
    }

    fn render_table_header(&self, content: String) -> Result<String> {
        Ok(format!("<thead>{}</thead>", content))
    }

    fn render_table_body(&self, content: String) -> Result<String> {
        Ok(format!("<tbody>{}</tbody>", content))
    }

    fn render_table_cell(&self, children: &[Node]) -> Result<String> {
        self.wrap_nodes("<td>", "</td>", children)
    }

    fn render_list_item(&self, checked: Option<bool>, children: &[Node]) -> Result<String> {
        if let Some(checked) = checked {
            if checked {
                self.wrap_nodes(
                    r#"<div class="d-flex fw-bold task-item">
                                <i class="text-success me-2 fal fa-check"></i>
                                <div>"#,
                    "</div></div>",
                    children,
                )
            } else {
                self.wrap_nodes(
                    r#"<div class="d-flex task-item">
                                <i class="text-danger me-2 fal fa-xmark"></i>
                                <div>"#,
                    "</div></div>",
                    children,
                )
            }
        } else {
            self.wrap_nodes("<li>", "</li>", children)
        }
    }

    fn render_paragraph(&self, children: &[Node]) -> Result<String> {
        self.wrap_nodes("<p>", "</p>", children)
    }
}

fn highlight_by_extension(path: &std::path::Path, s: &str) -> Result<Vec<String>> {
    let ss = syntect::parsing::SyntaxSet::load_defaults_newlines();
    let syn = ss
        .find_syntax_for_file(path)?
        .ok_or_else(|| Error::new("Syntax not found"))?;

    highlight_content(syn, s, &ss)
}
fn highlight(name: &str, s: &str) -> Result<Vec<String>> {
    let ss = syntect::parsing::SyntaxSet::load_defaults_newlines();
    let syn = ss
        .find_syntax_by_name(name)
        .ok_or_else(|| Error::new("Syntax not found"))?;
    highlight_content(syn, s, &ss)
}

fn highlight_content(
    syn: &syntect::parsing::SyntaxReference,
    s: &str,
    ss: &syntect::parsing::SyntaxSet,
) -> Result<Vec<String>> {
    let ts = syntect::highlighting::ThemeSet::load_defaults();
    let theme = ts
        .themes
        .get("Solarized (dark)")
        .ok_or_else(|| Error::new("Theme not found"))?;

    let mut h = syntect::easy::HighlightLines::new(syn, theme);
    let res = s
        .lines()
        .map(|s| {
            let hl = h.highlight_line(s, ss).unwrap();
            syntect::html::styled_line_to_highlighted_html(
                &hl[..],
                syntect::html::IncludeBackground::No,
            )
            .unwrap()
        })
        .collect::<Vec<_>>();
    Ok(res)
}

#[cfg(test)]
mod tests {
    use crate::{project::tests::project_fixture, RenderContext, Renderer};

    #[test]
    pub fn test_csv_table() {
        let result = "<table class=\"table table-sm table-striped\"><thead><tr><td>name</td><td>age</td><td>position</td></tr></thead><tbody><tr><td>alice</td><td>18</td><td>engineer</td></tr><tr><td>bob</td><td>19</td><td>engineer</td></tr><tr><td>charlie</td><td>20</td><td>manager</td></tr></tbody></table>";
        let project = project_fixture();
        let doc = project.get_document_for_url("/other/csv").unwrap();
        let renderer = super::HtmlRenderer {
            render_context: RenderContext::new(&project, doc),
        };
        assert_eq!(renderer.render_body().unwrap(), result);
    }
}
