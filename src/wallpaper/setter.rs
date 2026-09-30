use super::Wallpaper;
use super::error::WallpaperError;
use super::source::{LocalSource, WebSource};

#[allow(dead_code)]
pub trait WallpaperSetter {
    fn set(&self) -> Result<(), WallpaperError>;
}

impl WallpaperSetter for Wallpaper<LocalSource> {
    fn set(&self) -> Result<(), WallpaperError> {
        todo!("set wallpaper")
    }
}

impl<A> WallpaperSetter for Wallpaper<WebSource<A>> {
    fn set(&self) -> Result<(), WallpaperError> {
        let _image = &self.image;

        todo!("get bytes and set to wallpaper without download")
    }
}
