use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::{assets::CodexPath, Document, Result};

#[derive(Debug, Deserialize, Serialize, Default, Clone)]
#[serde(default)]
pub struct FolderDetails {
    pub name: Option<String>,
    pub menu_position: i32,
    pub menu_exclude: bool,
}

/// A folder in the project.
/// This is a recursive structure, so it can contain other folders.
/// It also contains a list of documents.
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Folder {
    pub name: String,
    pub path: CodexPath,
    pub documents: Vec<Document>,
    pub folders: Vec<Folder>,
    pub details: FolderDetails,
}

impl Folder {
    pub fn new(name: String, path: CodexPath) -> Self {
        Self {
            name,
            path,
            documents: vec![],
            folders: vec![],
            details: FolderDetails::default(),
        }
    }
    pub fn get_name(&self) -> String {
        let raw_name = self.details.name.clone().unwrap_or(self.name.clone());
        raw_name.replace("-", " ").replace("_", " ")
    }

    pub fn iter_all_documents<'a>(&'a self) -> Box<dyn Iterator<Item = &Document> + 'a> {
        Box::new(
            self.documents
                .iter()
                .chain(self.folders.iter().flat_map(|f| f.iter_all_documents())),
        )
    }
}

/// The project details.
/// This is the content of the `codex.yml` file.
#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(default)]
pub struct ProjectDetails {
    pub name: String,
    pub build_path: String,
    pub repo_url: Option<String>,
    pub project_page: Option<String>,
    pub base_url: String,
    pub author: Option<String>,
}

impl Default for ProjectDetails {
    fn default() -> Self {
        Self {
            name: String::from("New Project"),
            build_path: "dist".to_string(),
            repo_url: None,
            project_page: None,
            base_url: "/".to_string(),
            author: None,
        }
    }
}

/// The project.
/// This is the main structure that contains all the information about the project.
#[derive(Clone, Debug)]
pub struct Project {
    pub details: ProjectDetails,
    pub root_folder: Folder,
    pub path: CodexPath,
}

impl Default for Project {
    fn default() -> Self {
        Self {
            details: ProjectDetails::default(),
            path: CodexPath::current_dir(),
            root_folder: Folder::new(
                "Unnamed".to_string(),
                CodexPath::new(PathBuf::from("."), PathBuf::from(".")),
            ),
        }
    }
}

impl Project {
    pub fn load<P>(path: P, ignore_base_url: bool) -> Result<Self>
    where
        P: Into<PathBuf>,
    {
        let path = path.into();
        let mut project = if path.join("codex.yml").exists() {
            Project {
                details: serde_yaml::from_reader(std::fs::File::open(path.join("codex.yml"))?)?,
                ..Default::default()
            }
        } else {
            Project::default()
        };
        if ignore_base_url {
            project.details.base_url = "/".to_string();
        }
        if !project.details.base_url.ends_with('/') {
            project.details.base_url.push('/');
        }
        let proj_path = CodexPath::for_project(path.clone());
        project.path = proj_path.clone();
        project.root_folder.path = project.path.clone();
        project.root_folder = scan_folder(&proj_path)?;
        Ok(project)
    }

    pub fn reload(&mut self) -> Result<()> {
        let path = self.path.clone();
        self.root_folder = scan_folder(&path)?;
        Ok(())
    }

    pub fn get_document<P>(&self, path: P) -> Option<&Document>
    where
        P: Into<PathBuf> + std::convert::AsRef<std::path::Path>,
    {
        let path = path.into();
        self.root_folder
            .iter_all_documents()
            .find(|d| d.file_path.disk_path() == path)
    }

    pub fn get_document_for_url(&self, url: &str) -> Option<&Document> {
        let url = format!(
            "/{}",
            url.strip_prefix(&self.details.base_url).unwrap_or_default()
        );
        self.root_folder
            .iter_all_documents()
            .find(|d| d.file_path.document_url() == url)
    }
}

pub fn scan_folder(root_path: &CodexPath) -> Result<Folder> {
    let folder_name = root_path.basename().unwrap_or("Unnamed".to_string());
    let mut folder = Folder::new(folder_name, root_path.clone());
    folder.details = std::fs::File::open(root_path.disk_path().join("group.yml"))
        .ok()
        .and_then(|f| serde_yaml::from_reader(f).ok())
        .unwrap_or_default();
    for entry in root_path.disk_path().read_dir()? {
        let entry = entry?;
        let path = entry.path();
        if path.file_name().and_then(|s| s.to_str()) == Some("static") {
            continue;
        }
        if path.file_name().and_then(|s| s.to_str()) == Some("_internal") {
            continue;
        }
        if path.file_name().and_then(|s| s.to_str()) == Some("dist") {
            continue;
        }
        if path.file_name().and_then(|s| s.to_str()) == Some(".git") {
            continue;
        }

        if path.is_dir() {
            folder.folders.push(scan_folder(
                &root_path.new_path(path.to_path_buf().strip_prefix(&root_path.project_root)?),
            )?);
        } else if path.extension().and_then(|s| s.to_str()) == Some("md") {
            let file_path =
                root_path.new_path(path.to_path_buf().strip_prefix(&root_path.project_root)?);
            let document = Document::load(file_path)?;
            folder.documents.push(document);
        }
    }
    folder
        .folders
        .sort_by_key(|f| (f.details.menu_position, f.name.clone()));
    folder
        .documents
        .sort_by_key(|d| (d.frontmatter.menu_position, d.frontmatter.title.clone()));
    Ok(folder)
}

#[cfg(test)]

pub mod tests {
    use crate::{HtmlRenderer, RenderContext, Renderer};

    use super::*;

    pub fn project_fixture() -> Project {
        Project::load(PathBuf::from("test").join("fixture"), false).unwrap()
    }

    #[test]
    fn project_base_url() {
        let mut project = project_fixture();
        let doc = project
            .get_document_for_url("/elements/root_link")
            .unwrap()
            .clone();
        let render_context = RenderContext::new(&project, &doc);
        let renderer = HtmlRenderer { render_context };

        assert_eq!(
            doc.file_path.document_url(),
            "/elements/root_link".to_string()
        );
        assert_eq!(
            renderer.render_body().unwrap(),
            r#"<p><a href="/somewhere/someplace" alt="">Test</a></p>"#,
        );
        assert_eq!(project.details.base_url, "/".to_string());

        // With base URL
        project.details.base_url = "/docs/".to_string();
        project.reload().unwrap();
        let renderer = HtmlRenderer::new(RenderContext::new(&project, &doc));
        let doc = project
            .get_document_for_url("/docs/elements/root_link")
            .unwrap();
        assert_eq!(
            doc.file_path.document_url(),
            "/elements/root_link".to_string()
        );
        assert_eq!(
            renderer.render_body().unwrap(),
            r#"<p><a href="/docs/somewhere/someplace" alt="">Test</a></p>"#,
        );
        assert_eq!(project.details.base_url, "/docs/".to_string());

        let doc = project
            .get_document_for_url("/docs/elements/external_link")
            .unwrap();
        let renderer = HtmlRenderer::new(RenderContext::new(&project, doc));

        assert_eq!(
            renderer.render_body().unwrap(),
            r#"<p><a href="https://example.com" alt="">Test</a></p>"#,
        );
    }

    #[test]
    fn project_load_path() {
        project_fixture();
    }

    #[test]
    fn test_document_relative_path() {
        let project = project_fixture();
        let doc = project
            .get_document_for_url("/other/custom_component")
            .unwrap()
            .clone();

        assert_eq!(
            doc.file_path.relative_path,
            PathBuf::from("other").join("custom_component.md")
        );
    }

    #[test]
    fn component_override() {
        let project = project_fixture();
        let doc = project
            .get_document_for_url("/other/override_component")
            .unwrap()
            .clone();
        let render_context = RenderContext::new(&project, &doc);
        let renderer = HtmlRenderer { render_context };
        assert_eq!(renderer.render_body().unwrap().trim(), "Overridden");
    }

    #[test]
    fn custom_component() {
        let project = project_fixture();
        let doc = project
            .get_document_for_url("/other/custom_component")
            .unwrap()
            .clone();
        let render_context = RenderContext::new(&project, &doc);
        let renderer = HtmlRenderer { render_context };

        assert_eq!(renderer.render_body().unwrap().trim(), "hello Alice");
    }
}
