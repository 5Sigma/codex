use markdown::{mdast::Node, MdxExpressionKind, MdxSignal};

use crate::{assets::CodexPath, Project, Result};

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
    pub title: String,
    /// The subtitle of the document
    pub subtitle: Option<String>,
    /// A list of tags for the document
    pub tags: Vec<String>,
    pub menu_position: i32,
    /// Whether or not the document should be excluded from the site map
    pub menu_exclude: bool,
    /// A path to a JSON schema file that will be used to generate the
    /// document.
    pub json_schema: Option<String>,
}

/// A document or page in the project
/// This is used to represent a markdown file
#[derive(serde::Deserialize, serde::Serialize, Clone)]
pub struct Document {
    pub file_path: CodexPath,
    pub frontmatter: FrontMatter,
    pub url: String,
}

impl std::fmt::Debug for Document {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Document")
            .field("file_path", &self.file_path)
            .field("frontmatter", &self.frontmatter)
            .finish()
    }
}

impl Document {
    pub fn load(project: &Project, file_path: CodexPath) -> Result<Self> {
        // Generate the table of contents

        let ast = parse(&file_path)?;

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
            url: file_path.document_url(),
            file_path,
        })
    }
}

fn parse(file_path: &CodexPath) -> Result<Node> {
    let content = String::from_utf8(file_path.read()?.to_vec())?;

    // Parse the markdown into an AST
    parse_ast(&content)
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
