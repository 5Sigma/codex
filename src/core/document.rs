use std::collections::HashMap;

use handlebars::html_escape;
use markdown::{
    mdast::{AttributeContent, AttributeValue, Heading, MdxJsxAttribute, Node},
    MdxExpressionKind, MdxSignal,
};
use serde::{Deserialize, Serialize};
use syntect::{
    easy::HighlightLines,
    highlighting::ThemeSet,
    html::{styled_line_to_highlighted_html, IncludeBackground},
    parsing::{SyntaxReference, SyntaxSet},
};

use crate::{
    assets::CodexPath,
    json_schema::{build_example, parse_schema},
    render_template, Error, Project, Result,
};

/// The front matter of a document
/// This is used to store metadata about a document
/// # See Also
/// * [`Document::frontmatter`] - The front matter of a document
/// * [`Document::parse`] - Parse a markdown file into a document
/// * [`Document::parse_file`] - Parse a markdown file into a document
#[derive(Debug, serde::Deserialize, serde::Serialize, Clone, Default)]
#[serde(default)]
pub struct FrontMatter {
    /// The title of the document
    title: String,
    /// The subtitle of the document
    subtitle: Option<String>,
    /// A list of tags for the document
    tags: Vec<String>,
    menu_position: i32,
    /// Whether or not the document should be excluded from the site map
    menu_exclude: bool,
    /// A path to a JSON schema file that will be used to generate the
    /// document.
    json_schema: Option<String>,
}

/// A document or page in the project
/// This is used to represent a markdown file
#[derive(serde::Deserialize, serde::Serialize, Clone)]
pub struct Document {
    /// The path to the document
    pub file_path: CodexPath,
    /// The front matter of the document
    pub frontmatter: FrontMatter,
    /// The table of contents of the document
    pub toc: Vec<TocEntry>,
    /// The URL of the document
    pub base_url: String,
    pub url: String,
}

impl std::fmt::Debug for Document {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Document")
            .field("file_path", &self.file_path)
            .field("frontmatter", &self.frontmatter)
            .field("base_url", &self.base_url)
            .finish()
    }
}

impl Document {
    fn parse(file_path: &CodexPath) -> Result<Node> {
        let content = String::from_utf8(file_path.read()?.to_vec())?;

        // Parse the markdown into an AST
        parse_ast(&content)
    }

    pub fn load(project: &Project, file_path: CodexPath) -> Result<Self> {
        // Generate the table of contents

        let ast = Self::parse(&file_path)?;
        let toc = toc(ast
            .children()
            .ok_or(crate::Error::new("Invalid markdown".to_string()))?);

        // Extract the front matter
        let frontmatter = ast
            .children()
            .and_then(|children| {
                children.iter().find_map(|child| {
                    if let Node::Yaml(yml) = child {
                        serde_yaml::from_str::<FrontMatter>(&yml.value).ok()
                    } else {
                        None
                    }
                })
            })
            .unwrap_or_default();

        Ok(Document {
            frontmatter,
            toc,
            url: file_path.document_url(),
            base_url: project.details.base_url.clone(),
            file_path,
        })
    }

    pub fn last_modified(&self) -> Result<String> {
        let file_time = std::fs::metadata(self.file_path.disk_path())?.modified()?;
        let dt: chrono::DateTime<chrono::Local> = chrono::DateTime::from(file_time);
        Ok(dt.to_utc().format("%Y-%m-%dT%H:%M:%S%z").to_string())
    }

    pub fn body(&self) -> Result<String> {
        let ast = Self::parse(&self.file_path)?;
        let body = self.to_html(&ast);
        Ok(body)
    }

    /// Get the HTML content of the page
    pub fn page_content(&self, project: &crate::Project) -> Result<String> {
        let sitemap = (&project.root_folder).into();
        let mut data = DataContext {
            body: self.body()?,
            document: self.frontmatter.clone(),
            sitemap,
            project: project.details.clone(),
            toc: self.toc.clone(),
            modified: self.last_modified().ok(),
        };

        if let Some(ref schema_file) = self.frontmatter.json_schema {
            data.body.push_str("<h4 class=\"mt-4\">Fields</h4>");
            data.body
                .push_str(&self.component_json_schema_fields(HashMap::from([(
                    "file".to_string(),
                    schema_file.to_string(),
                )]))?);
            data.body.push_str("<h4 class=\"mt-4\">Example</h4>");
            data.body
                .push_str(&self.component_json_schema_example(HashMap::from([(
                    "file".to_string(),
                    schema_file.to_string(),
                )]))?);
        }

        render_template(
            data,
            &String::from_utf8(
                project
                    .path
                    .new_path("_internal/templates/article.html")
                    .read()?
                    .to_vec(),
            )?,
        )
    }

    /// Convert a single node to HTML
    pub fn to_html(&self, node: &Node) -> String {
        match node {
            Node::Root(root) => root.children.iter().fold(String::new(), |acc, child| {
                format!("{}{}", acc, self.to_html(child))
            }),
            Node::BlockQuote(block_quote) => self.wrap_nodes(
                r#"<blockquote class="blockquote">"#,
                "</blockquote>",
                &block_quote.children,
            ),
            Node::FootnoteDefinition(_) => "".to_string(),
            Node::MdxJsxFlowElement(el) => {
                if let Some(name) = el.name.as_ref() {
                    match self.apply_component(name, &el.attributes, &el.children) {
                        Ok(html) => html,
                        Err(e) => format!("<pre>{}</pre>", html_escape(&e.to_string())),
                    }
                } else {
                    "".to_string()
                }
            }
            Node::MdxJsxTextElement(el) => {
                if let Some(name) = el.name.as_ref() {
                    match self.apply_component(name, &el.attributes, &el.children) {
                        Ok(html) => html,
                        Err(e) => format!("<pre>{}</pre>", html_escape(&e.to_string())),
                    }
                } else {
                    "".to_string()
                }
            }
            Node::List(list) => {
                if list.ordered {
                    self.wrap_nodes("<ol>", "</ol>", &list.children)
                } else {
                    self.wrap_nodes("<ul>", "</ul>", &list.children)
                }
            }
            Node::MdxjsEsm(_) => "".to_string(),
            Node::Toml(_) => "".to_string(),
            Node::Yaml(_) => "".to_string(),
            Node::Break(_) => "".to_string(),
            Node::InlineCode(c) => {
                format!(r#"<code class="inline">{}</code>"#, c.value)
            }
            Node::InlineMath(_) => "".to_string(),
            Node::Delete(d) => self.wrap_nodes(
                "<span style=\"text-decoration: line-through\">",
                "</span>",
                &d.children,
            ),
            Node::Emphasis(em) => {
                self.wrap_nodes(r#"<span class="fst-italic">"#, "</span>", &em.children)
            }
            Node::MdxTextExpression(_) => "".to_string(),
            Node::FootnoteReference(_) => "".to_string(),
            Node::Html(_) => "".to_string(),
            Node::Image(img) => {
                format!(
                    r#"<img class="img-fluid" src="{}" title="{}""#,
                    img.url,
                    img.title.clone().unwrap_or_default()
                )
            }
            Node::ImageReference(_) => "".to_string(),
            Node::Link(link) => {
                if link.url.starts_with('/') {
                    format!(
                        "<a href=\"{}{}\">{}</a>",
                        self.base_url,
                        link.url.clone().trim_start_matches('/'),
                        self.all_to_html(link.children.as_slice())
                    )
                } else {
                    format!(
                        "<a href=\"{}\">{}</a>",
                        link.url.clone().trim_start_matches('/'),
                        self.all_to_html(link.children.as_slice())
                    )
                }
            }
            Node::LinkReference(_) => "".to_string(),
            Node::Strong(bold) => {
                self.wrap_nodes(r#"<span class="fw-bold">"#, "</span>", &bold.children)
            }
            Node::Text(text) => text.value.clone(),
            Node::Code(code) => {
                let lines = if let Some(ref lang) = code.lang {
                    highlight(lang, code.value.trim()).unwrap_or(
                        html_escape(&code.value)
                            .lines()
                            .map(|s| s.to_string())
                            .collect::<Vec<_>>(),
                    )
                } else {
                    html_escape(&code.value)
                        .lines()
                        .map(|s| s.to_string())
                        .collect::<Vec<_>>()
                };
                let template = self.file_path.new_path("_internal/templates/code.html");
                let data = CodeContext {
                    lines,
                    lang: code.lang.clone().unwrap_or_default(),
                };
                template
                    .read()
                    .map(|d| d.to_vec())
                    .map_err(crate::Error::from)
                    .and_then(|d| {
                        render_template(data, &String::from_utf8(d).unwrap_or_default())
                            .map_err(crate::Error::from)
                    })
                    .unwrap_or_else(|e| format!("<pre>{}</pre>", html_escape(&e.to_string())))
            }
            Node::Math(_) => "".to_string(),
            Node::MdxFlowExpression(exp) => self
                .apply_expression(&exp.value)
                .unwrap_or_else(|e| format!("<pre>{}</pre>", html_escape(&e.to_string()))),
            Node::Heading(h) => {
                let Some(text) = heading_text(h) else {
                    return "<pre>No header text found</pre>".to_string();
                };
                let slug = slug(&text);
                let tag = format!("h{}", h.depth + 3);
                let html = h.children.iter().fold(String::new(), |acc, child| {
                    format!("{}{}", acc, self.to_html(child))
                });
                format!("<{} class=\"mt-4\" id=\"{}\">{}</{}>", tag, slug, html, tag)
            }
            Node::Table(table) => self.wrap_nodes(
                "<table class=\"table table-sm\">",
                "</table>",
                &table.children,
            ),
            Node::ThematicBreak(_) => "<hr/>".to_string(),
            Node::TableRow(node) => self.wrap_nodes("<tr>", "</tr>", &node.children),
            Node::TableCell(node) => self.wrap_nodes("<td>", "</td>", &node.children),
            Node::ListItem(li) => {
                if let Some(checked) = li.checked {
                    if checked {
                        self.wrap_nodes(
                            r#"<div class="d-flex fw-bold task-item">
                                <i class="text-success me-2 fal fa-check"></i>
                                <div>"#,
                            "</div></div>",
                            &li.children,
                        )
                    } else {
                        self.wrap_nodes(
                            r#"<div class="d-flex task-item">
                                <i class="text-danger me-2 fal fa-xmark"></i>
                                <div>"#,
                            "</div></div>",
                            &li.children,
                        )
                    }
                } else {
                    self.wrap_nodes("<li>", "</li>", &li.children)
                }
            }
            Node::Definition(_) => "".to_string(),
            Node::Paragraph(p) => self.wrap_nodes("<p>", "</p>", &p.children),
        }
    }

    /// Wrap a list of nodes in HTML with the provided start and end fragments
    pub fn wrap_nodes(&self, start: &str, end: &str, nodes: &[Node]) -> String {
        format!("{}{}{}", start, self.all_to_html(nodes), end)
    }

    /// Convert a list of nodes to HTML
    pub fn all_to_html(&self, nodes: &[Node]) -> String {
        nodes.iter().fold(String::new(), |acc, child| {
            format!("{}{}", acc, self.to_html(child))
        })
    }

    pub fn apply_component(
        &self,
        name: &str,
        attrs: &[markdown::mdast::AttributeContent],
        children: &[Node],
    ) -> Result<String> {
        let mut data = convert_component_attributes(attrs);
        match name {
            "CsvTable" => {
                let csv_file_name = self.file_path.new_path(
                    data.get("file")
                        .ok_or_else(|| Error::new("No file specified"))?,
                );
                let has_headers = data.get("headers").unwrap_or(&"true".to_string()) == "true";
                let mut reader = csv::ReaderBuilder::new()
                    .has_headers(has_headers)
                    .from_path(csv_file_name.disk_path())?;

                #[derive(Debug, Serialize)]
                struct CsvCtx {
                    headers: Vec<String>,
                    rows: Vec<Vec<String>>,
                }

                let ctx = CsvCtx {
                    headers: if has_headers {
                        reader
                            .headers()?
                            .iter()
                            .map(|s| s.to_string())
                            .collect::<Vec<_>>()
                    } else {
                        vec![]
                    },
                    rows: reader
                        .records()
                        .flat_map(|s| {
                            s.map(|i| i.iter().map(|s| s.to_string()).collect::<Vec<_>>())
                        })
                        .collect::<Vec<_>>(),
                };

                let cmp_path = self
                    .file_path
                    .new_path("_internal/components/csv_table.html");
                render_template(ctx, &String::from_utf8(cmp_path.read()?.to_vec())?)
            }
            "JsonSchemaFields" => self.component_json_schema_fields(data),
            "JsonSchemaExample" => self.component_json_schema_example(data),
            "CodeFile" => {
                let source_file_path = self.file_path.new_path(
                    data.get("file")
                        .ok_or_else(|| Error::new("No file specified"))?,
                );

                #[derive(Debug, Serialize)]
                struct CodeFileCtx {
                    lines: Vec<String>,
                    collapse: bool,
                    lang: String,
                }

                let cmp_path = self.file_path.new_path("_internal/templates/code.html");
                let lines = highlight_by_extension(
                    &source_file_path.disk_path(),
                    &String::from_utf8(source_file_path.read()?.to_vec())?,
                )?;
                let ctx = CodeFileCtx {
                    lines,
                    collapse: data.get("collapsed").unwrap_or(&"false".to_string()) == "true",
                    lang: "".to_string(),
                };
                render_template(ctx, &String::from_utf8(cmp_path.read()?.to_vec())?)
            }
            _ => {
                let cmp_path = self
                    .file_path
                    .new_path(format!("_internal/components/{}.html", name.to_lowercase()));

                if cmp_path.exists() {
                    data.insert("children".to_string(), self.all_to_html(children));
                    render_template(data, &String::from_utf8(cmp_path.read()?.to_vec())?)
                } else {
                    Ok("<pre>Unknown Component</pre>".to_string())
                }
            }
        }
    }

    pub fn apply_expression(&self, exp: &str) -> Result<String> {
        match exp {
            "id" => {
                let alphabet: [char; 52] = [
                    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p',
                    'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', 'A', 'B', 'C', 'D', 'E', 'F',
                    'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V',
                    'W', 'X', 'Y', 'Z',
                ];
                Ok(nanoid::nanoid!(5, &alphabet))
            }
            _ => Ok("".to_string()),
        }
    }

    fn component_json_schema_example(&self, data: HashMap<String, String>) -> Result<String> {
        let schema_filename = self.file_path.new_path(
            data.get("file")
                .ok_or_else(|| Error::new("No file specified"))?,
        );
        let schema_str = schema_filename.read()?;
        let content = build_example(&schema_str)?;

        #[derive(Debug, Serialize)]
        struct CodeFileCtx {
            lines: Vec<String>,
            collapse: bool,
            lang: String,
        }

        let cmp_path = self.file_path.new_path("_internal/templates/code.html");
        let lines = highlight("JSON", &content)?;
        let ctx = CodeFileCtx {
            lines,
            collapse: data.get("collapsed").unwrap_or(&"false".to_string()) == "true",
            lang: "".to_string(),
        };
        render_template(ctx, &String::from_utf8(cmp_path.read()?.to_vec())?)
    }

    fn component_json_schema_fields(&self, data: HashMap<String, String>) -> Result<String> {
        let schema_filename = self.file_path.new_path(
            data.get("file")
                .ok_or_else(|| Error::new("No file specified"))?,
        );
        let data = schema_filename.read()?;
        let fields = parse_schema(&data)?;

        let mut output = String::new();
        for mut field in fields.into_iter() {
            field.children = self.to_html(&parse_ast(&field.children)?);
            let cmp_path = self.file_path.new_path("_internal/components/field.html");
            output.push_str(&render_template(
                field,
                &String::from_utf8(cmp_path.read()?.to_vec())?,
            )?);
        }

        Ok(output)
    }
}

fn parse_ast(content: &str) -> Result<Node> {
    let parser_options = markdown::ParseOptions {
        constructs: markdown::Constructs {
            code_indented: false,
            frontmatter: true,
            mdx_jsx_flow: true,
            html_flow: false,
            html_text: false,
            mdx_esm: true,
            mdx_expression_flow: true,
            mdx_expression_text: true,
            gfm_task_list_item: true,
            gfm_strikethrough: true,
            mdx_jsx_text: true,
            gfm_table: true,
            ..Default::default()
        },
        mdx_expression_parse: Some(Box::new(parse_expression)),
        gfm_strikethrough_single_tilde: true,
        math_text_single_dollar: true,
        ..markdown::ParseOptions::mdx()
    };
    Ok(markdown::to_mdast(content, &parser_options)?)
}

pub fn parse_expression(_value: &str, _kind: &MdxExpressionKind) -> MdxSignal {
    MdxSignal::Ok
}

fn convert_component_attributes(
    attrs: &[markdown::mdast::AttributeContent],
) -> HashMap<String, String> {
    let mut data = HashMap::new();
    for attr in attrs {
        if let AttributeContent::Property(MdxJsxAttribute {
            name,
            value: Some(AttributeValue::Literal(value)),
        }) = attr
        {
            data.insert(name.to_string(), value.to_string());
        }
    }
    data
}

/// Generate the table of contents for a list of nodes
/// This will find all the headings in the list of nodes and generate a table of contents
/// # Arguments
/// * `nodes` - The list of nodes to generate the table of contents for
/// # Returns
/// A list of table of contents entries
fn toc(nodes: &[Node]) -> Vec<TocEntry> {
    let mut s = vec![];
    for node in nodes.iter() {
        if let Node::Heading(heading) = node {
            if let Some(text) = heading_text(heading) {
                let sg = slug(&text);
                s.push(TocEntry {
                    depth: heading.depth,
                    text,
                    slug: sg,
                });
            }
        }
    }
    s
}

/// Get the text of a heading node
/// This will find the first text node in the heading and return its value
/// # Arguments
/// * `node` - The heading node to get the text of
/// # Returns
/// The text of the heading node
pub fn heading_text(node: &Heading) -> Option<String> {
    node.children.iter().find_map(|child| {
        if let Node::Text(text) = child {
            Some(text.value.clone())
        } else {
            None
        }
    })
}

/// Convert a string to a slug
/// This is used for generating IDs for headings
/// It converts the string to lowercase, replaces spaces with dashes, and removes punctuation
/// # Arguments
/// * `text` - The text to convert to a slug
/// # Returns
/// The slug version of the text
pub fn slug(text: &str) -> String {
    text.to_lowercase().replace(' ', "-").replace(
        [
            ':', '?', '!', '.', ',', ';', '(', ')', '[', ']', '{', '}', '\'', '\\', '/', '\\', '<',
            '>', '|',
        ],
        "",
    )
}

/// The data context for a page
/// This is used to render the page template
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct DataContext {
    pub document: FrontMatter,
    pub sitemap: SiteMapFolder,
    pub body: String,
    pub project: crate::ProjectDetails,
    pub toc: Vec<TocEntry>,
    pub modified: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct CodeContext {
    lines: Vec<String>,
    lang: String,
}

/// A table of contents entry
/// This is used to generate the table of contents for a page
/// # See Also
/// * [`Document::toc`] - Generate the table of contents for a document
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TocEntry {
    pub depth: u8,
    pub text: String,
    pub slug: String,
}

/// A folder in the site map
/// This is used to generate the site map
/// # See Also
/// * [`crate::Project::site_map`] - Generate the site map for a project
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct SiteMapFolder {
    pub name: String,
    pub folders: Vec<SiteMapFolder>,
    pub pages: Vec<Document>,
    pub menu_position: i32,
}

impl From<&crate::Folder> for SiteMapFolder {
    fn from(folder: &crate::Folder) -> Self {
        let mut pages = folder
            .documents
            .iter()
            .filter(|d| !d.frontmatter.menu_exclude)
            .cloned()
            .collect::<Vec<_>>();

        pages.sort_by_key(|p| (p.frontmatter.menu_position, p.frontmatter.title.clone()));

        let mut folders = folder
            .folders
            .iter()
            .filter(|f| !f.details.menu_exclude)
            .map(SiteMapFolder::from)
            .collect::<Vec<_>>();

        folders.sort_by_key(|f| (f.menu_position, f.name.clone()));

        SiteMapFolder {
            folders,
            pages,
            menu_position: folder.details.menu_position,
            name: folder.get_name(),
        }
    }
}

pub fn highlight_by_extension(path: &std::path::Path, s: &str) -> Result<Vec<String>> {
    let ss = SyntaxSet::load_defaults_newlines();
    let syn = ss
        .find_syntax_for_file(path)?
        .ok_or_else(|| Error::new("Syntax not found"))?;

    highlight_content(syn, s, &ss)
}
pub fn highlight(name: &str, s: &str) -> Result<Vec<String>> {
    let ss = SyntaxSet::load_defaults_newlines();
    let syn = ss
        .find_syntax_by_name(name)
        .ok_or_else(|| Error::new("Syntax not found"))?;
    highlight_content(syn, s, &ss)
}

pub fn highlight_content(syn: &SyntaxReference, s: &str, ss: &SyntaxSet) -> Result<Vec<String>> {
    let ts = ThemeSet::load_defaults();
    let theme = ts
        .themes
        .get("Solarized (dark)")
        .ok_or_else(|| Error::new("Theme not found"))?;

    let mut h = HighlightLines::new(syn, theme);
    let res = s
        .lines()
        .map(|s| {
            let hl = h.highlight_line(s, ss).unwrap();
            styled_line_to_highlighted_html(&hl[..], IncludeBackground::No).unwrap()
        })
        .collect::<Vec<_>>();
    Ok(res)
}

#[cfg(test)]
mod tests {
    use crate::tests::project_fixture;

    #[test]
    pub fn test_csv_table() {
        let project = project_fixture();
        let doc = project.get_document_for_url("/other/csv").unwrap();

        let result = "<table class=\"table table-sm table-striped\">\n  <thead>\n    <tr>\n        <th class=\"text-uppercase\">name</th>\n        <th class=\"text-uppercase\">age</th>\n        <th class=\"text-uppercase\">position</th>\n    </tr>\n  </thead>\n  <tr>\n      <td>alice</td>\n      <td>18</td>\n      <td>engineer</td>\n  </tr>\n  <tr>\n      <td>bob</td>\n      <td>19</td>\n      <td>engineer</td>\n  </tr>\n  <tr>\n      <td>charlie</td>\n      <td>20</td>\n      <td>manager</td>\n  </tr>\n</table>\n";
        assert_eq!(doc.body().unwrap(), result.to_string());
    }
}
