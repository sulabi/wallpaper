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
    type Source;
    async fn set_wallpaper(self) -> Result<Wallpaper<Self::Source>, WallpaperError>;
}

impl WallpaperSetter for Wallpaper<LocalSource> {
    type Source = MemorySource<LocalSource>;

    async fn set_wallpaper(self) -> Result<Wallpaper<Self::Source>, WallpaperError> {
        todo!("set wallpaper")
    }
}

impl<A> WallpaperSetter for Wallpaper<WebSource<A>> {
    type Source = MemorySource<WebSource<A>>;

    async fn set_wallpaper(self) -> Result<Wallpaper<Self::Source>, WallpaperError> {
        let wallpaper = self.fetch().await?;

        wallpaper.set_wallpaper().await
    }
}
