use std::collections::HashMap;

use handlebars::html_escape;
use markdown::{
    mdast::{AttributeContent, AttributeValue, Heading, MdxJsxAttribute, Node},
    MdxExpressionKind, MdxSignal,
};
use serde::{Deserialize, Serialize};
use tree_sitter_highlight::{HighlightConfiguration, HighlightEvent, Highlighter};

use crate::{assets::CodexPath, render_template, Project, Result};

/// The names of the classes for syntax highlighting
/// This is used to highlight code blocks
/// # See Also
/// * [`build_highlighter_config`] - Build a highlighter configuration for the given language
const HIGHLIGHT_NAMES: &[&str] = &[
    "attribute",
    "constant",
    "function.builtin",
    "function",
    "keyword",
    "operator",
    "property",
    "punctuation",
    "punctuation.bracket",
    "punctuation.delimiter",
    "string",
    "string.special",
    "tag",
    "type",
    "type.builtin",
    "variable",
    "variable.builtin",
    "variable.parameter",
    "number",
    "true",
    "false",
];

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
}

/// A document or page in the project
/// This is used to represent a markdown file
#[derive(serde::Deserialize, serde::Serialize, Clone)]
pub struct Document {
    /// The path to the document
    pub file_path: CodexPath,
    /// The front matter of the document
    pub frontmatter: FrontMatter,
    /// The HTML body of the document
    pub body: String,
    /// The table of contents of the document
    pub toc: Vec<TocEntry>,
    /// The URL of the document
    pub base_url: String,
    // url of the page
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
    /// Parse a markdown file into a documents
    /// This will parse the front matter, generate the table of contents, and convert the markdown to HTML
    /// # Arguments
    /// * `file_path` - The path to the markdown file!
    /// * `content` - The content of the markdown file
    /// # Returns
    /// A `Result` containing the parsed document
    /// # Errors
    /// This will return an error if the markdown is invalid or if the front matter is invalid
    /// # See Also
    /// * [`Document::parse_file`] - Parse a markdown file into a document
    pub fn parse(project: &Project, file_path: CodexPath, content: &str) -> Result<Self> {
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
                mdx_jsx_text: true,
                gfm_table: true,
                ..Default::default()
            },
            mdx_expression_parse: Some(Box::new(parse_expression)),
            gfm_strikethrough_single_tilde: true,
            math_text_single_dollar: true,
            ..markdown::ParseOptions::mdx()
        };

        // Parse the markdown into an AST
        let ast = markdown::to_mdast(content, &parser_options)?;
        // Generate the table of contents
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

        let mut doc = Document {
            frontmatter,
            body: "".to_string(),
            toc,
            url: file_path.document_url(),
            base_url: project.details.base_url.clone(),
            file_path,
        };
        // Convert the AST to HTML
        doc.body = doc.to_html(&ast);

        Ok(doc)
    }

    /// Parse a markdown file into a document
    /// This will parse the front matter, generate the table of contents, and convert the markdown to HTML
    /// # Arguments
    /// * `file_path` - The path to the markdown file!
    /// # Returns
    /// A `Result` containing the parsed document
    /// # Errors
    /// This will return an error if the markdown is invalid or if the front matter is invalid
    /// # Examples
    pub fn parse_file(project: &Project, file_path: CodexPath) -> Result<Self> {
        let content = String::from_utf8(file_path.read()?.to_vec())?;
        Self::parse(project, file_path, &content)
    }

    /// Get the HTML content of the page
    pub fn page_content(&self, project: &crate::Project) -> Result<String> {
        let sitemap = (&project.root_folder).into();
        let data = DataContext {
            body: self.body.clone(),
            document: self.frontmatter.clone(),
            sitemap,
            project: project.details.clone(),
            toc: self.toc.clone(),
        };
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
            Node::InlineCode(_) => "".to_string(),
            Node::InlineMath(_) => "".to_string(),
            Node::Delete(_) => "".to_string(),
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
                let mut s = String::from("<pre><code>");
                if let Some(config) = code
                    .lang
                    .as_ref()
                    .and_then(|lang| build_highlighter_config(lang))
                {
                    let mut highlighter = Highlighter::new();
                    let highlights = highlighter
                        .highlight(&config, code.value.as_bytes(), None, |_| None)
                        .unwrap();

                    for event in highlights {
                        match event.unwrap() {
                            HighlightEvent::Source { start, end } => {
                                s.push_str(&html_escape(&code.value[start..end]));
                            }
                            HighlightEvent::HighlightStart(highlight) => {
                                s.push_str(&format!("<span class=\"highlight-{}\">", highlight.0));
                            }
                            HighlightEvent::HighlightEnd => {
                                s.push_str("</span>");
                            }
                        }
                    }
                } else {
                    s.push_str(&html_escape(&code.value));
                }
                s.push_str("</code></pre>");
                s
            }
            Node::Math(_) => "".to_string(),
            Node::MdxFlowExpression(exp) => self.apply_expression(&exp.value).unwrap(),
            Node::Heading(h) => {
                let text = heading_text(h).unwrap();
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
            Node::ListItem(li) => self.wrap_nodes("<li>", "</li>", &li.children),
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
        let cmp_path = self
            .file_path
            .new_path(format!("_internal/components/{}.html", name.to_lowercase()));

        if cmp_path.exists() {
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
            data.insert("children".to_string(), self.all_to_html(children));
            render_template(data, &String::from_utf8(cmp_path.read()?.to_vec())?)
        } else {
            Ok("<pre>Unknown Component</pre>".to_string())
        }
    }

    pub fn apply_expression(&self, exp: &str) -> Result<String> {
        match exp {
            "id" => Ok(nanoid::nanoid!()),
            _ => Ok("".to_string()),
        }
    }
}

pub fn parse_expression(_value: &str, _kind: &MdxExpressionKind) -> MdxSignal {
    MdxSignal::Ok
}

/// Build a highlighter configuration for the given language
fn build_highlighter_config(lang: &str) -> Option<HighlightConfiguration> {
    let mut config = match lang {
        "bash" | "sh" => HighlightConfiguration::new(
            tree_sitter_bash::language(),
            tree_sitter_bash::HIGHLIGHT_QUERY,
            "",
            "",
        )
        .ok(),
        "html" => HighlightConfiguration::new(
            tree_sitter_html::language(),
            tree_sitter_html::HIGHLIGHT_QUERY,
            "",
            "",
        )
        .ok(),
        "json" => HighlightConfiguration::new(
            tree_sitter_json::language(),
            tree_sitter_json::HIGHLIGHT_QUERY,
            "",
            "",
        )
        .ok(),
        "c" => HighlightConfiguration::new(
            tree_sitter_c::language(),
            tree_sitter_c::HIGHLIGHT_QUERY,
            "",
            "",
        )
        .ok(),
        "cpp" | "c++" => HighlightConfiguration::new(
            tree_sitter_cpp::language(),
            tree_sitter_cpp::HIGHLIGHT_QUERY,
            "",
            "",
        )
        .ok(),
        "rust" => HighlightConfiguration::new(
            tree_sitter_rust::language(),
            tree_sitter_rust::HIGHLIGHT_QUERY,
            "",
            "",
        )
        .ok(),
        "javascript" | "js" => HighlightConfiguration::new(
            tree_sitter_javascript::language(),
            tree_sitter_javascript::HIGHLIGHT_QUERY,
            "",
            "",
        )
        .ok(),
        "python" => HighlightConfiguration::new(
            tree_sitter_python::language(),
            tree_sitter_python::HIGHLIGHT_QUERY,
            "",
            "",
        )
        .ok(),
        "lua" => HighlightConfiguration::new(
            tree_sitter_lua::language(),
            tree_sitter_lua::HIGHLIGHTS_QUERY,
            "",
            "",
        )
        .ok(),
        "css" => HighlightConfiguration::new(
            tree_sitter_css::language(),
            tree_sitter_css::HIGHLIGHTS_QUERY,
            "",
            "",
        )
        .ok(),
        "" => None,
        _ => None,
    };
    if let Some(ref mut c) = config {
        c.configure(HIGHLIGHT_NAMES)
    }
    config
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
        let mut pages = folder.documents.clone();

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
