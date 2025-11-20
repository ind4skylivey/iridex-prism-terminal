use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "themes/"]
pub struct ThemeAssets;
