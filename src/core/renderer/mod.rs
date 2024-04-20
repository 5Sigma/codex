#![allow(unused_variables)]
mod html;
pub use html::HtmlRenderer;
mod latex;
pub use latex::LatexRenderer;

use crate::{assets::CodexPath, error::Result, Document, FrontMatter, Project};
use markdown::mdast::{
    AttributeContent, AttributeValue, MdxJsxAttribute, Node, TableCell, TableRow,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub struct RenderContext<'a> {
    pub project: &'a Project,
    pub document: &'a Document,
}

impl<'a> RenderContext<'a> {
    pub fn new(project: &'a Project, document: &'a Document) -> Self {
        Self { project, document }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TocEntry {
    pub depth: u8,
    pub text: String,
    pub slug: String,
}

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

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct DataContext {
    pub document: FrontMatter,
    pub sitemap: SiteMapFolder,
    pub body: String,
    pub project: crate::ProjectDetails,
    pub toc: Vec<TocEntry>,
    pub modified: Option<String>,
    pub current_url: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct CodeContext {
    lines: Vec<String>,
    lang: String,
}

pub trait Renderer {
    fn get_context(&self) -> &RenderContext;
    fn finalize_render(&self, data: DataContext) -> Result<String>;
    fn render_body(&self) -> Result<String> {
        let ast = self.parse(&self.get_context().document.file_path)?;
        let body = self.render_node(&ast)?;
        Ok(body)
    }

    fn render(&self) -> Result<String> {
        let ctx = self.get_context();
        let sitemap = (&ctx.project.root_folder).into();

        let mut data = DataContext {
            body: self.render_body()?,
            document: ctx.document.frontmatter.clone(),
            sitemap,
            current_url: ctx.document.url.clone(),
            project: ctx.project.details.clone(),
            toc: self
                .parse(&ctx.document.file_path)?
                .children()
                .map(|v| self.toc(v))
                .unwrap_or_default(),
            modified: self.last_modified().ok(),
        };

        if let Some(ref schema_file) = ctx.document.frontmatter.json_schema {
            data.body.push_str(&self.render_heading(
                1,
                &[Node::Text(markdown::mdast::Text {
                    value: "Fields".to_string(),
                    position: Default::default(),
                })],
            )?);

            data.body.push_str(&self.handle_jsx_element(
                "JsonSchemaFields",
                HashMap::from([("file".to_string(), schema_file.to_string())]),
                &[],
            )?);
            data.body.push_str(&self.render_heading(
                1,
                &[Node::Text(markdown::mdast::Text {
                    value: "Example".to_string(),
                    position: Default::default(),
                })],
            )?);
            data.body.push_str(&self.handle_jsx_element(
                "JsonSchemaExample",
                HashMap::from([("file".to_string(), schema_file.to_string())]),
                &[],
            )?);
        }

        self.finalize_render(data)
    }

    fn render_nodes(&self, nodes: &[Node]) -> Result<String> {
        nodes.iter().try_fold(String::new(), |acc, child| {
            self.render_node(child)
                .map(|child| format!("{}{}", acc, child))
        })
    }

    fn render_blockquote(&self, children: &[Node]) -> Result<String> {
        Ok(String::new())
    }

    fn render_footnote_definition(
        &self,
        identifier: &str,
        label: &Option<String>,
    ) -> Result<String> {
        Ok(String::new())
    }

    fn render_jsx_element(
        &self,
        name: &str,
        attrs: HashMap<String, String>,
        children: &[Node],
    ) -> Result<String> {
        Ok(String::new())
    }

    fn render_expression(&self, value: &str) -> Result<String> {
        Ok(String::new())
    }

    fn render_list(&self, ordered: bool, children: &[Node]) -> Result<String> {
        Ok(String::new())
    }

    fn render_inline_code(&self, code: &str) -> Result<String> {
        Ok(String::new())
    }

    fn render_delete(&self, children: &[Node]) -> Result<String> {
        Ok(String::new())
    }
    fn render_emphasis(&self, children: &[Node]) -> Result<String> {
        Ok(String::new())
    }

    fn render_bold(&self, children: &[Node]) -> Result<String> {
        Ok(String::new())
    }

    fn render_image(&self, url: &str, title: Option<String>) -> Result<String> {
        Ok(String::new())
    }

    fn render_link(&self, url: &str, title: Option<String>, children: &[Node]) -> Result<String> {
        Ok(String::new())
    }

    fn render_text(&self, text: &str) -> Result<String> {
        Ok(text.to_string())
    }

    fn render_code(
        &self,
        code: &str,
        lang: Option<String>,
        filepath: Option<std::path::PathBuf>,
    ) -> Result<String> {
        Ok(String::new())
    }
    fn render_heading(&self, depth: u8, children: &[Node]) -> Result<String> {
        Ok(String::new())
    }
    fn render_table_header(&self, content: String) -> Result<String> {
        Ok(String::new())
    }
    fn render_table_body(&self, content: String) -> Result<String> {
        Ok(String::new())
    }
    fn render_table(&self, content: String) -> Result<String> {
        Ok(String::new())
    }
    fn render_thematic_break(&self) -> Result<String> {
        Ok(String::new())
    }

    fn render_table_row(&self, children: &[Node]) -> Result<String> {
        Ok(String::new())
    }
    fn render_table_cell(&self, children: &[Node]) -> Result<String> {
        Ok(String::new())
    }

    fn render_list_item(&self, checked: Option<bool>, children: &[Node]) -> Result<String> {
        Ok(String::new())
    }
    fn render_paragraph(&self, children: &[Node]) -> Result<String> {
        Ok(String::new())
    }

    fn handle_jsx_element(
        &self,
        name: &str,
        attrs: HashMap<String, String>,
        children: &[Node],
    ) -> Result<String> {
        match name {
            "JsonSchemaFields" => {
                let schema_filename = self.get_context().document.file_path.new_path(
                    attrs
                        .get("file")
                        .ok_or_else(|| crate::Error::new("No file specified"))?,
                );
                let data = schema_filename.read()?;
                let fields = crate::json_schema::parse_schema(&data)?;
                let mut output = String::new();
                for field in fields.into_iter() {
                    let mut attrs = HashMap::from([
                        ("name".to_string(), field.name),
                        ("type".to_string(), field.data_type),
                        ("children".to_string(), field.children),
                        ("required".to_string(), field.required.to_string()),
                    ]);
                    if field.deprecated {
                        attrs.insert("deprecated".to_string(), "true".to_string());
                    }
                    output.push_str(&self.render_jsx_element("Field", attrs, children)?);
                }
                Ok(output)
            }
            "JsonSchemaExample" => {
                let schema_filename = self.get_context().document.file_path.new_path(
                    attrs
                        .get("file")
                        .ok_or_else(|| crate::Error::new("No file specified"))?,
                );
                let schema_str = schema_filename.read()?;
                let content = crate::json_schema::build_example(&schema_str)?;

                self.render_code(&content, Some("JSON".to_string()), None)
            }
            "CodeFile" => {
                let source_file_path = self.get_context().document.file_path.new_path(
                    attrs
                        .get("file")
                        .ok_or_else(|| crate::Error::new("No file specified"))?,
                );

                self.render_code(
                    &String::from_utf8(source_file_path.read()?.to_vec())?,
                    None,
                    Some(source_file_path.disk_path()),
                )
            }
            "CsvTable" => {
                let csv_file_name = self.get_context().document.file_path.new_path(
                    attrs
                        .get("file")
                        .ok_or_else(|| crate::Error::new("No file specified"))?,
                );

                let mut reader = csv::ReaderBuilder::new()
                    .has_headers(false)
                    .from_path(csv_file_name.disk_path())?;

                let rows = reader
                    .records()
                    .map(|r| {
                        Node::TableRow(TableRow {
                            children: r
                                .unwrap()
                                .iter()
                                .map(|c| {
                                    Node::TableCell(TableCell {
                                        children: vec![Node::Text(markdown::mdast::Text {
                                            value: c.to_string(),
                                            position: Default::default(),
                                        })],
                                        position: Default::default(),
                                    })
                                })
                                .collect::<Vec<_>>(),
                            position: Default::default(),
                        })
                    })
                    .collect::<Vec<_>>();

                self.handle_table(&rows)
            }
            _ => self.render_jsx_element(name, attrs, children),
        }
    }

    fn render_node(&self, node: &markdown::mdast::Node) -> Result<String> {
        let ctx = self.get_context();
        match node {
            Node::Root(root) => self.render_nodes(&root.children),
            Node::BlockQuote(block_quote) => self.render_blockquote(&block_quote.children),
            Node::FootnoteDefinition(f) => self.render_footnote_definition(&f.identifier, &f.label),
            Node::MdxJsxFlowElement(el) => {
                let attrs = self.convert_component_attributes(&el.attributes);
                let name = el
                    .name
                    .as_ref()
                    .ok_or_else(|| crate::Error::new("Missing name"))?;

                self.handle_jsx_element(name, attrs, &el.children)
            }
            Node::MdxJsxTextElement(el) => {
                let attrs = self.convert_component_attributes(&el.attributes);
                let name = el
                    .name
                    .as_ref()
                    .ok_or_else(|| crate::Error::new("Missing name"))?;

                self.handle_jsx_element(name, attrs, &el.children)
            }
            Node::List(list) => self.render_list(list.ordered, &list.children),
            Node::MdxjsEsm(_) => Ok("".to_string()),
            Node::Toml(_) => Ok("".to_string()),
            Node::Yaml(_) => Ok("".to_string()),
            Node::Break(_) => Ok("".to_string()),
            Node::InlineCode(c) => self.render_inline_code(&c.value),
            Node::InlineMath(_) => Ok("".to_string()),
            Node::Delete(d) => self.render_delete(&d.children),
            Node::Emphasis(em) => self.render_emphasis(&em.children),
            Node::MdxTextExpression(_) => Ok("".to_string()),
            Node::FootnoteReference(_) => Ok("".to_string()),
            Node::Html(_) => Ok("".to_string()),
            Node::Image(img) => self.render_image(&img.url, img.title.clone()),
            Node::ImageReference(_) => Ok("".to_string()),
            Node::Link(link) => {
                let url = if link.url.starts_with('/') {
                    format!(
                        "{}{}",
                        ctx.project.details.base_url,
                        link.url.clone().trim_start_matches('/'),
                    )
                } else {
                    link.url.clone()
                };
                self.render_link(&url, link.title.clone(), &link.children)
            }
            Node::LinkReference(_) => Ok("".to_string()),
            Node::Strong(bold) => self.render_bold(&bold.children),
            Node::Text(text) => self.render_text(&text.value),
            Node::Code(code) => self.render_code(&code.value, code.lang.clone(), None),
            Node::Math(_) => Ok("".to_string()),
            Node::MdxFlowExpression(exp) => self.render_expression(&exp.value),
            Node::Heading(h) => self.render_heading(h.depth, &h.children),
            Node::Table(table) => self.handle_table(&table.children),
            Node::ThematicBreak(_) => self.render_thematic_break(),
            Node::TableRow(ref node) => self.render_table_row(&node.children),

            Node::TableCell(node) => self.render_table_cell(&node.children),
            Node::ListItem(li) => self.render_list_item(li.checked, &li.children),
            Node::Definition(_) => Ok("".to_string()),
            Node::Paragraph(p) => self.render_paragraph(&p.children),
        }
    }

    fn handle_table(&self, children: &[Node]) -> Result<String> {
        let mut i = children.iter().filter(|i| matches!(i, Node::TableRow(_)));
        let Some(header_row) = i.next() else {
            return Ok(String::new());
        };
        let rows = i.cloned().collect::<Vec<_>>();

        let mut out = String::new();
        out.push_str(&self.render_table_header(self.render_node(header_row)?)?);
        out.push_str(&self.render_table_body(self.render_nodes(&rows)?)?);
        self.render_table(out)
    }

    fn convert_component_attributes(
        &self,
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

    fn slug(&self, text: &str) -> String {
        text.to_lowercase().replace(' ', "-").replace(
            [
                ':', '?', '!', '.', ',', ';', '(', ')', '[', ']', '{', '}', '\'', '\\', '/', '\\',
                '<', '>', '|',
            ],
            "",
        )
    }

    fn parse(&self, file_path: &CodexPath) -> Result<Node> {
        let content = String::from_utf8(file_path.read()?.to_vec())?;

        // Parse the markdown into an AST
        match self.parse_ast(&content) {
            Ok(ast) => Ok(ast),
            Err(e) => {
                self.parse_ast(&format!(r#"<Alert title="Parsing error" style="danger">{}</Alert>"#, e.to_string()))
            }
        }
    }

    fn parse_ast(&self, content: &str) -> Result<Node> {
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

    fn get_text(&self, nodes: &[Node]) -> Option<String> {
        nodes.iter().find_map(|child| {
            if let Node::Text(text) = child {
                Some(text.value.clone())
            } else {
                None
            }
        })
    }

    fn last_modified(&self) -> Result<String> {
        let file_time =
            std::fs::metadata(self.get_context().document.file_path.disk_path())?.modified()?;
        let dt: chrono::DateTime<chrono::Local> = chrono::DateTime::from(file_time);
        Ok(dt.to_utc().format("%Y-%m-%dT%H:%M:%S%z").to_string())
    }

    fn toc(&self, nodes: &[Node]) -> Vec<TocEntry> {
        let mut s = vec![];
        for node in nodes.iter() {
            if let Node::Heading(heading) = node {
                if let Some(text) = self.get_text(&heading.children) {
                    let sg = self.slug(&text);
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
}

#[allow(dead_code)]
fn parse_expression(_value: &str, _kind: &markdown::MdxExpressionKind) -> markdown::MdxSignal {
    markdown::MdxSignal::Ok
}

#[cfg(test)]
pub mod tests {}
