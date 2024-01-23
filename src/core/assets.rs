use crate::{Error, Project, Result};
use std::{borrow::Cow, path::PathBuf};

use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "assets"]
struct EmbeddedAsset;

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

pub fn static_files(project: &Project) -> Result<Vec<StaticAsset>> {
    let mut assets = EmbeddedAsset::iter()
        .filter(|f| f.starts_with("static/"))
        .map(StaticAsset::Internal)
        .collect::<Vec<_>>();

    if project.path.join("static").exists() {
        for entry in walkdir::WalkDir::new(project.path.join("static")) {
            let entry = entry?;
            if entry.file_type().is_file() {
                let p = entry.path().strip_prefix(&project.path)?;
                assets.push(StaticAsset::External(p.to_path_buf()));
            }
        }
    }

    Ok(assets)
}

#[derive(Debug)]
pub enum StaticAsset<'a> {
    Internal(Cow<'a, str>),
    External(PathBuf),
}

impl<'a> StaticAsset<'a> {
    pub fn source_path(&self) -> PathBuf {
        match self {
            StaticAsset::Internal(s) => PathBuf::from(s.to_string()),
            StaticAsset::External(s) => s.clone(),
        }
    }
    pub fn write(&self, project: &Project, destination: PathBuf) -> Result<()> {
        if let Some(parent) = destination.parent() {
            if !parent.exists() {
                std::fs::create_dir_all(parent)?;
            }
        }
        match self {
            StaticAsset::Internal(src) => {
                let f =
                    EmbeddedAsset::get(src).ok_or_else(|| Error::new("Static file not found"))?;

                std::fs::write(destination, f.data)?;
            }
            StaticAsset::External(src) => {
                std::fs::copy(project.path.join(src), destination)?;
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::Project;

    #[test]
    fn test_static_assets() {
        let project = Project::load("test/fixture", false).unwrap();
        let assets = crate::assets::static_files(&project).unwrap();
        dbg!(&assets);
        assert!(assets
            .iter()
            .map(|a| a.source_path().display().to_string())
            .collect::<Vec<_>>()
            .contains(&"test/fixture/static/nested/object".to_string()));
    }
}
