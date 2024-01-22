use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::{Document, Result};

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
    pub fn get_name(&self) -> String {
        self.details.name.clone().unwrap_or(self.name.clone())
    }

    pub fn scan(&mut self, root_path: &PathBuf) -> Result<()> {
        self.documents.clear();
        self.folders.clear();
        self.details = std::fs::File::open(self.path.join("group.yml"))
            .ok()
            .and_then(|f| serde_yaml::from_reader(f).ok())
            .unwrap_or_default();
        let p = std::path::Path::new(&self.path);
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
                let mut folder = Folder {
                    name: entry.file_name().to_str().unwrap().to_string(),
                    path: path.to_path_buf(),
                    documents: Vec::new(),
                    folders: Vec::new(),
                    details: FolderDetails::default(),
                };
                folder.scan(root_path)?;
                self.folders.push(folder);
            } else if path.extension().and_then(|s| s.to_str()) == Some("md") {
                let mut document = Document::parse_file(path.clone())?;
                document.file_path = path.to_path_buf();
                if path.file_name().and_then(|s| s.to_str()) == Some("index.md") {
                    document.url = PathBuf::from("/")
                        .join(path.parent().unwrap().strip_prefix(root_path)?)
                        .display()
                        .to_string();
                } else {
                    document.url = PathBuf::from("/")
                        .join(path.strip_prefix(root_path)?)
                        .with_extension("")
                        .display()
                        .to_string();
                }
                self.documents.push(document);
            }
        }
        Ok(())
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
}

impl Default for ProjectDetails {
    fn default() -> Self {
        Self {
            name: String::from("New Project"),
            build_path: "dist".to_string(),
            repo_url: None,
            project_page: None,
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
    pub fn load<P>(path: P) -> Result<Self>
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
        project.path = path.clone();
        project.root_folder.path = path.clone();
        project.root_folder.scan(&path)?;
        Ok(project)
    }

    pub fn reload(&mut self) -> Result<()> {
        self.root_folder.scan(&self.path)?;
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
}
