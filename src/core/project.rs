use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::{Document, Error, Result};

#[derive(Debug, Deserialize, Serialize, Default)]
#[serde(default)]
pub struct FolderDetails {
    pub name: Option<String>,
    pub menu_position: i32,
    pub menu_exclude: bool,
}

/// A folder in the project.
/// This is a recursive structure, so it can contain other folders.
/// It also contains a list of documents.
#[derive(Debug, Deserialize, Serialize, Default)]
pub struct Folder {
    pub name: String,
    pub path: PathBuf,
    pub documents: Vec<Document>,
    pub folders: Vec<Folder>,
    pub details: FolderDetails,
}

impl Folder {
    pub fn new(name: String, path: PathBuf) -> Self {
        Self {
            name,
            path,
            ..Default::default()
        }
    }
    pub fn get_name(&self) -> String {
        self.details.name.clone().unwrap_or(self.name.clone())
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
}

impl Default for ProjectDetails {
    fn default() -> Self {
        Self {
            name: String::from("New Project"),
            build_path: "dist".to_string(),
            repo_url: None,
            project_page: None,
            base_url: "/".to_string(),
        }
    }
}

/// The project.
/// This is the main structure that contains all the information about the project.
pub struct Project {
    pub details: ProjectDetails,
    pub root_folder: Folder,
    pub path: PathBuf,
}

impl Default for Project {
    fn default() -> Self {
        Self {
            details: ProjectDetails::default(),
            path: PathBuf::from("."),
            root_folder: Folder {
                name: String::from("."),
                path: PathBuf::from("."),
                ..Default::default()
            },
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
        project.path = path.clone();
        project.root_folder.path = path.clone();
        project.root_folder = project.scan_folder(&path)?;
        Ok(project)
    }

    pub fn reload(&mut self) -> Result<()> {
        let path = self.path.clone();
        self.root_folder = self.scan_folder(&path)?;
        Ok(())
    }

    pub fn get_document<P>(&self, path: P) -> Option<&Document>
    where
        P: Into<PathBuf> + std::convert::AsRef<std::path::Path>,
    {
        let path = path.into();
        self.root_folder
            .iter_all_documents()
            .find(|d| d.file_path == path)
    }

    pub fn get_document_for_url(&self, url: &str) -> Option<&Document> {
        self.root_folder.iter_all_documents().find(|d| d.url == url)
    }

    pub fn scan_folder(&mut self, root_path: &PathBuf) -> Result<Folder> {
        let folder_name = root_path
            .file_name()
            .ok_or_else(|| Error::new("Bad folder name"))?
            .to_str()
            .ok_or_else(|| Error::new("Bad folder name"))?;
        let mut folder = Folder::new(folder_name.into(), root_path.clone());
        folder.details = std::fs::File::open(root_path.join("group.yml"))
            .ok()
            .and_then(|f| serde_yaml::from_reader(f).ok())
            .unwrap_or_default();
        let p = std::path::Path::new(&root_path);
        for entry in p.read_dir()? {
            let entry = entry?;
            let path = entry.path();
            if path.file_name().and_then(|s| s.to_str()) == Some("static") {
                continue;
            }
            if path.file_name().and_then(|s| s.to_str()) == Some("dist") {
                continue;
            }
            if path.file_name().and_then(|s| s.to_str()) == Some(".git") {
                continue;
            }

            if path.is_dir() {
                folder.folders.push(self.scan_folder(&path.to_path_buf())?);
            } else if path.extension().and_then(|s| s.to_str()) == Some("md") {
                let mut document = Document::parse_file(self, path.clone())?;
                document.file_path = path.to_path_buf();
                let rel_doc_path = if path.file_name().and_then(|s| s.to_str()) == Some("index.md")
                {
                    PathBuf::from(&self.details.base_url)
                        .join(path.parent().unwrap().strip_prefix(&self.path)?)
                } else {
                    PathBuf::from(&self.details.base_url)
                        .join(path.strip_prefix(&self.path)?)
                        .with_extension("")
                };
                document.url = format!(
                    "/{}",
                    rel_doc_path
                        .strip_prefix("/")?
                        .components()
                        .map(|c| c.as_os_str().to_str().unwrap())
                        .collect::<Vec<&str>>()
                        .join("/")
                );
                document.base_url = self.details.base_url.clone();
                folder.documents.push(document);
            }
        }
        Ok(folder)
    }
}

#[cfg(test)]

pub mod tests {
    use super::*;

    #[test]
    fn project_base_url() {
        let mut project = Project::load("test/fixture", false).unwrap();

        // Without base url
        let doc = project.get_document_for_url("/elements/root_link").unwrap();
        assert_eq!(doc.url, "/elements/root_link".to_string());
        assert_eq!(
            doc.body,
            r#"<p><a href="/somewhere/someplace">Test</a></p>"#,
        );
        assert_eq!(project.details.base_url, "/".to_string());

        // With base url
        project.details.base_url = "/docs/".to_string();
        project.reload().unwrap();
        let doc = project
            .get_document_for_url("/docs/elements/root_link")
            .unwrap();
        assert_eq!(doc.url, "/docs/elements/root_link".to_string());
        assert_eq!(
            doc.body,
            r#"<p><a href="/docs/somewhere/someplace">Test</a></p>"#,
        );
        assert_eq!(project.details.base_url, "/docs/".to_string());

        let doc = project
            .get_document_for_url("/docs/elements/external_link")
            .unwrap();

        assert_eq!(doc.body, r#"<p><a href="https://example.com">Test</a></p>"#,);
    }
}
