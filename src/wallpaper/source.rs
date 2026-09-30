use super::Wallpaper;
use super::error::WallpaperError;

pub struct WebSource {
    pub url: String,
}

pub struct LocalSource {
    pub path: std::path::PathBuf,
}

impl Wallpaper<WebSource> {
    pub async fn download(self) -> Result<Wallpaper<LocalSource>, WallpaperError> {
        todo!("download and convert to localwallpaper")
    }
}

impl Wallpaper<LocalSource> {
    pub fn delete(self) -> Result<(), WallpaperError> {
        todo!("delete and nothing")
    }
}
