use std::borrow::Cow;

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

pub fn static_files() -> impl Iterator<Item = Cow<'static, str>> {
    EmbeddedAsset::iter().filter(|f| f.starts_with("static/"))
}
