use super::Wallpaper;
use super::source::{WebSource, LocalSource};

pub struct WallpaperSearcher {}
pub struct LocalSearcher {}

impl WallpaperSearcher {
    pub async fn search(&self) -> Vec<Wallpaper<WebSource>> {
        todo!()
    }
}

impl LocalSearcher {
    pub async fn search(&self) -> Vec<Wallpaper<LocalSource>> {
        todo!()
    }
}
