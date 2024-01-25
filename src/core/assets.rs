use crate::{Error, Project, Result};
use std::{
    borrow::Cow,
    path::{Path, PathBuf},
};

use rust_embed::RustEmbed;
use serde::Serialize;

#[derive(RustEmbed)]
#[folder = "assets"]
pub struct EmbeddedAsset;

pub fn get_str(name: &str) -> String {
    EmbeddedAsset::get(name)
        .and_then(|f| String::from_utf8(f.data.to_vec()).ok())
        .unwrap_or_default()
}

pub fn get_bytes(name: &str) -> Cow<'static, [u8]> {
    EmbeddedAsset::get(name).unwrap().data
}

pub fn exists(name: &str) -> bool {
    EmbeddedAsset::get(name).is_some()
}

pub fn static_files(project: &Project) -> Result<Vec<CodexPath>> {
    let mut assets = EmbeddedAsset::iter()
        .filter(|f| f.starts_with("static"))
        .map(|f| project.path.new_path(f.to_string()))
        .collect::<Vec<_>>();
    if project.path.join("static").exists() {
        for entry in walkdir::WalkDir::new(project.path.disk_path().join("static")) {
            let entry = entry?;
            if entry.file_type().is_file() {
                assets.push(project.path.new_path(entry.path()));
            }
        }
    }

    Ok(assets)
}

#[derive(Debug, Eq, PartialEq, Clone, Serialize)]
pub struct CodexPath {
    project_root: PathBuf,
    relative_path: PathBuf,
}

impl CodexPath {
    pub fn new<P>(project_root: P, relative_path: P) -> Self
    where
        P: Into<PathBuf>,
    {
        Self {
            project_root: project_root.into(),
            relative_path: relative_path.into(),
        }
    }

    pub fn current_dir() -> Self {
        Self {
            project_root: PathBuf::from("."),
            relative_path: PathBuf::from("."),
        }
    }

    pub fn new_path<P>(&self, relative_path: P) -> Self
    where
        P: Into<PathBuf>,
    {
        let relative_path: PathBuf = relative_path.into();
        let rel_path = if relative_path.starts_with(&self.project_root) {
            relative_path.strip_prefix(&self.project_root).unwrap()
        } else {
            &relative_path
        };

        Self {
            project_root: self.project_root.clone(),
            relative_path: rel_path.to_path_buf(),
        }
    }

    pub fn basename(&self) -> Option<String> {
        self.disk_path()
            .file_name()
            .and_then(|s| s.to_str())
            .map(|s| s.to_string())
    }

    pub fn for_project<P>(project_root: P) -> Self
    where
        P: Into<PathBuf> + Clone,
    {
        Self {
            project_root: project_root.into(),
            relative_path: PathBuf::from("."),
        }
    }

    pub fn from_url(&self, url: &str) -> Self {
        Self {
            project_root: self.project_root.clone(),
            relative_path: url.split('/').fold(PathBuf::from(""), |acc, s| acc.join(s)),
        }
    }

    pub fn join<P>(&self, subpath: P) -> CodexPath
    where
        P: AsRef<Path>,
    {
        CodexPath {
            project_root: self.project_root.clone(),
            relative_path: self.relative_path.join(subpath),
        }
    }

    pub fn relative_to(&self, p: &Path) -> PathBuf {
        p.join(&self.relative_path)
    }

    pub fn disk_path(&self) -> PathBuf {
        self.project_root.join(&self.relative_path)
    }

    pub fn exists_on_disk(&self) -> bool {
        self.disk_path().exists()
    }

    pub fn exists_embedded(&self) -> bool {
        EmbeddedAsset::get(&self.relative_path.display().to_string()).is_some()
    }

    pub fn exists(&self) -> bool {
        self.exists_on_disk() || self.exists_embedded()
    }

    pub fn root_url(&self) -> String {
        let url = self
            .relative_path
            .components()
            .map(|c| c.as_os_str().to_string_lossy())
            .collect::<Vec<_>>()
            .join("/");
        format!("/{}", url.trim_start_matches('/'))
    }

    pub fn root_url_with_base(&self, base_url: &str) -> String {
        format!(
            "/{}/{}",
            base_url.trim_matches('/'),
            self.root_url().trim_start_matches('/'),
        )
    }

    pub fn document_url(&self) -> String {
        let url = if self.is_index() {
            self.relative_path
                .parent()
                .unwrap()
                .components()
                .map(|c| c.as_os_str().to_string_lossy())
                .collect::<Vec<_>>()
                .join("/")
        } else {
            self.relative_path
                .with_extension("")
                .components()
                .map(|c| c.as_os_str().to_string_lossy())
                .collect::<Vec<_>>()
                .join("/")
        };
        format!("/{}", url.trim_start_matches('/'))
    }

    pub fn document_url_with_base(&self, base_url: &str) -> String {
        format!(
            "/{}/{}",
            base_url.trim_matches('/'),
            self.document_url().trim_start_matches('/'),
        )
    }

    pub fn read(&self) -> Result<Cow<[u8]>> {
        if self.exists_on_disk() {
            Ok(Cow::Owned(std::fs::read(self.disk_path())?))
        } else if self.exists_embedded() {
            Ok(EmbeddedAsset::get(&self.relative_path.to_string_lossy())
                .unwrap()
                .data)
        } else {
            Err(Error::new("Asset not found"))
        }
    }

    pub fn is_index(&self) -> bool {
        self.relative_path.file_name().and_then(|s| s.to_str()) == Some("index.md")
    }

    pub fn set_root(&mut self, root_path: &Path) {
        self.project_root = root_path.to_path_buf();
    }

    pub fn write(&self, destination_root: &Path, strip: PathBuf) -> Result<()> {
        let dst = self
            .project_root
            .join(destination_root)
            .join(self.relative_path.strip_prefix(strip)?);

        if let Some(parent) = dst.parent() {
            if !parent.exists() {
                std::fs::create_dir_all(parent)?;
            }
        }

        if self.exists_on_disk() {
            std::fs::copy(self.disk_path(), dst)?;
        } else {
            std::fs::write(
                dst,
                EmbeddedAsset::get(&self.relative_path.to_string_lossy())
                    .ok_or_else(|| Error::new("Asset not found"))?
                    .data,
            )?;
        }
        Ok(())
    }
}

impl<'de> ::serde::Deserialize<'de> for CodexPath {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(Self {
            project_root: PathBuf::from("."),
            relative_path: PathBuf::from(s),
        })
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use crate::Project;

    use super::CodexPath;

    #[test]
    fn test_static_assets() {
        let project = Project::load(PathBuf::from("test").join("fixture"), false).unwrap();
        let assets = crate::assets::static_files(&project).unwrap();
        assert!(assets
            .iter()
            .map(|a| dbg!(a.disk_path().display().to_string()))
            .collect::<Vec<_>>()
            .contains(
                &PathBuf::from("test")
                    .join("fixture")
                    .join("static")
                    .join("nested")
                    .join("object")
                    .display()
                    .to_string()
            ));
    }

    #[test]
    fn test_codex_path() {
        let cp = CodexPath::new(
            PathBuf::from("parent").join("root"),
            PathBuf::from("one").join("two").join("afile.txt"),
        );

        assert_eq!(
            cp.disk_path(),
            PathBuf::from("parent")
                .join("root")
                .join("one")
                .join("two")
                .join("afile.txt")
        );

        assert_eq!(cp.root_url(), "/one/two/afile.txt");
        assert_eq!(cp.root_url_with_base("base"), "/base/one/two/afile.txt");
        assert_eq!(
            cp.relative_to(&PathBuf::from("dist")),
            PathBuf::from("dist/one/two/afile.txt")
        );
    }

    #[test]
    fn test_codex_path_index() {
        let cp = CodexPath::new(
            PathBuf::from("parent").join("root"),
            PathBuf::from("one").join("two").join("index.md"),
        );
        assert!(cp.is_index());
    }

    #[test]
    fn test_codex_path_document_url() {
        let cp = CodexPath::new(
            PathBuf::from("parent").join("root"),
            PathBuf::from("one").join("two").join("index.md"),
        );
        assert_eq!(cp.document_url(), "/one/two");

        let cp = CodexPath::new(
            PathBuf::from("parent").join("root"),
            PathBuf::from("one").join("two").join("another.md"),
        );
        assert_eq!(cp.document_url(), "/one/two/another");
    }
}
