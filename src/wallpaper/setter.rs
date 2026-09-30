use super::Wallpaper;
use super::error::WallpaperError;
use super::source::{LocalSource, WebSource};

pub trait WallpaperSetter {
    fn set(&self) -> Result<(), WallpaperError>;
}

impl WallpaperSetter for Wallpaper<LocalSource> {
    fn set(&self) -> Result<(), WallpaperError> {
        todo!("set wallpaper")
    }
}

impl WallpaperSetter for Wallpaper<WebSource> {
    fn set(&self) -> Result<(), WallpaperError> {
        todo!("get bytes and set to wallpaper without download")
    }
}
