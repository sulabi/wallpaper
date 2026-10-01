use std::path::PathBuf;

use crate::wallpaper::source::MemorySource;

use super::Wallpaper;
use super::error::WallpaperError;
use super::source::{LocalSource, WebSource};

#[allow(dead_code)]
#[allow(async_fn_in_trait)]
pub trait WallpaperSetter {
    async fn set_wallpaper(&self) -> Result<(), WallpaperError>;
}

impl WallpaperSetter for Wallpaper<LocalSource> {
    async fn set_wallpaper(&self) -> Result<(), WallpaperError> {
        todo!("set wallpaper")
    }
}

impl<A> WallpaperSetter for Wallpaper<WebSource<A>> {
    async fn set_wallpaper(&self) -> Result<(), WallpaperError> {
        let wallpaper = self.fetch().await?;

        wallpaper.set_wallpaper().await
    }
}

impl WallpaperSetter for Wallpaper<MemorySource> {
    async fn set_wallpaper(&self) -> Result<(), WallpaperError> {
        let tmpfile = tempfile::NamedTempFile::new()?;
        std::fs::write(tmpfile.path(), &self.source.bytes)?;

        // TODO: make this a wrapper and configurable for other OS
        // TODO: also allow other args
        let status = std::process::Command::new("awww")
            .args(["img", tmpfile.path().to_str().unwrap()])
            .status()?;

        if !status.success() {
            return Err(WallpaperError::SetterError(status));
        }

        Ok(())
    }
}

#[allow(dead_code)]
impl Wallpaper<MemorySource> {
    async fn download(&self) -> Result<PathBuf, WallpaperError> {
        todo!("return downloaded path from config")
    }
}
