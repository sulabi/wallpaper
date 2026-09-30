use super::Wallpaper;
use super::error::WallpaperError;
use super::source::{LocalSource, WebSource};

pub struct WebSearcher {}
pub struct LocalSearcher {}

impl WebSearcher {
    pub async fn search(&self) -> Result<Vec<Wallpaper<WebSource>>, WallpaperError> {
        todo!()
    }
}

impl LocalSearcher {
    pub async fn search(&self) -> Vec<Wallpaper<LocalSource>> {
        todo!()
    }
}
