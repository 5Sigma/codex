# Heading generator
Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Et leo duis ut diam quam nulla porttitor. Quam viverra orci sagittis eu volutpat odio facilisis mauris sit. Semper risus in hendrerit gravida. Aliquam id diam maecenas ultricies mi. Arcu odio ut sem nulla pharetra diam. Massa placerat duis ultricies lacus sed turpis tincidunt. Lobortis scelerisque fermentum dui faucibus in ornare quam. Vivamus arcu felis bibendum ut. Commodo ullamcorper a lacus vestibulum sed arcu non odio euismod. Sed ullamcorper morbi tincidunt ornare massa. Sapien pellentesque habitant morbi tristique senectus et netus et malesuada. At tellus at urna condimentum mattis pellentesque id nibh tortor. Vitae et leo duis ut diam. At quis risus sed vulputate odio ut enim.

Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Et leo duis ut diam quam nulla porttitor. Quam viverra orci sagittis eu volutpat odio facilisis mauris sit. Semper risus in hendrerit gravida. Aliquam id diam maecenas ultricies mi. Arcu odio ut sem nulla pharetra diam. Massa placerat duis ultricies lacus sed turpis tincidunt. Lobortis scelerisque fermentum dui faucibus in ornare quam. Vivamus arcu felis bibendum ut. Commodo ullamcorper a lacus vestibulum sed arcu non odio euismod. Sed ullamcorper morbi tincidunt ornare massa. Sapien pellentesque habitant morbi tristique senectus et netus et malesuada. At tellus at urna condimentum mattis pellentesque id nibh tortor. Vitae et leo duis ut diam. At quis risus sed vulputate odio ut enim.

Senectus et netus et malesuada fames ac turpis. Scelerisque felis imperdiet proin fermentum leo. Enim sit amet venenatis urna cursus eget nunc scelerisque. A arcu cursus vitae congue mauris. Mauris pellentesque pulvinar pellentesque habitant morbi tristique. Lacus sed turpis tincidunt id aliquet risus feugiat. Id nibh tortor id aliquet lectus proin nibh. Semper quis lectus nulla at volutpat diam ut venenatis. Nec nam aliquam sem et tortor consequat id porta. Orci phasellus egestas tellus rutrum.

## Second level depth
Scelerisque purus semper eget duis at tellus at urna. Etiam tempor orci eu lobortis. Integer feugiat scelerisque varius morbi enim nunc faucibus a pellentesque. Amet tellus cras adipiscing enim eu turpis egestas. Tempor orci eu lobortis elementum. Quam lacus suspendisse faucibus interdum posuere lorem ipsum dolor sit. Nisl vel pretium lectus quam id leo in vitae turpis. Blandit volutpat maecenas volutpat blandit aliquam etiam erat velit. Porta nibh venenatis cras sed. Ut faucibus pulvinar elementum integer enim neque volutpat ac tincidunt. Gravida in fermentum et sollicitudin ac orci. Dictumst vestibulum rhoncus est pellentesque elit ullamcorper dignissim. Dictumst quisque sagittis purus sit amet volutpat consequat mauris nunc.

## Second level depth
Lacus vestibulum sed arcu non odio euismod lacinia. Eu tincidunt tortor aliquam nulla facilisi cras fermentum. Consectetur libero id faucibus nisl tincidunt eget. Neque gravida in fermentum et sollicitudin. Auctor urna nunc id cursus metus aliquam. Iaculis nunc sed augue lacus viverra vitae congue eu consequat. Id consectetur purus ut faucibus pulvinar elementum integer enim neque. Non odio euismod lacinia at quis risus. Facilisi cras fermentum odio eu feugiat pretium nibh. Scelerisque purus semper eget duis. Vitae purus faucibus ornare suspendisse sed nisi lacus sed. Cursus eget nunc scelerisque viverra mauris in aliquam. Ullamcorper sit amet risus nullam. Magna fringilla urna porttitor rhoncus dolor purus non enim praesent. Aliquam sem fringilla ut morbi tincidunt augue interdum. Morbi non arcu risus quis varius.

### Third level depth
Augue interdum velit euismod in pellentesque massa placerat duis ultricies. Vitae semper quis lectus nulla at volutpat diam ut. Risus nec feugiat in fermentum posuere urna nec. Sollicitudin ac orci phasellus egestas tellus rutrum tellus pellentesque. Pharetra sit amet aliquam id diam maecenas ultricies. Egestas integer eget aliquet nibh. Duis ut diam quam nulla porttitor massa id neque aliquam. Maecenas ultricies mi eget mauris. Quam viverra orci sagittis eu volutpat. Etiam sit amet nisl purus in mollis. Lorem mollis aliquam ut porttitor leo a diam sollicitudin tempor. Massa id neque aliquam vestibulum morbi blandit cursus risus.


```Rust
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
```

<Field 
    name="type" 
    type="String" 
    type_link="https://en.wikipedia.org/wiki/String_(computer_science)">
The type name of the field. This can be omitted if not applicable.
</Field>
<Field 
    name="type" 
    type="String" 
    type_link="https://en.wikipedia.org/wiki/String_(computer_science)">
The type name of the field. This can be omitted if not applicable.
</Field>
<Field 
    name="type" 
    type="String" 
    type_link="https://en.wikipedia.org/wiki/String_(computer_science)">
The type name of the field. This can be omitted if not applicable.
</Field>
<Field 
    name="type" 
    type="String" 
    type_link="https://en.wikipedia.org/wiki/String_(computer_science)">
The type name of the field. This can be omitted if not applicable.
</Field>
<Field 
    name="type" 
    type="String" 
    type_link="https://en.wikipedia.org/wiki/String_(computer_science)">
The type name of the field. This can be omitted if not applicable.
</Field>
<Field 
    name="type" 
    type="String" 
    type_link="https://en.wikipedia.org/wiki/String_(computer_science)">
The type name of the field. This can be omitted if not applicable.
</Field>
<Field 
    name="type" 
    type="String" 
    type_link="https://en.wikipedia.org/wiki/String_(computer_science)">
The type name of the field. This can be omitted if not applicable.
</Field>
<Field 
    name="type" 
    type="String" 
    type_link="https://en.wikipedia.org/wiki/String_(computer_science)">
The type name of the field. This can be omitted if not applicable.
</Field>
