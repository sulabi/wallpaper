use super::{
    Wallpaper,
    error::WallpaperError,
    source::{LocalSource, MemorySource, WebSource},
};

#[cfg(target_os = "linux")]
mod linux;

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
