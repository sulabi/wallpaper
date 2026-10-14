use crate::{WallpaperSource, source::Fetch};

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
    type Source = LocalSource;

    async fn set_wallpaper(self) -> Result<Wallpaper<Self::Source>, WallpaperError> {
        todo!("set wallpaper")
    }
}

impl<S> WallpaperSetter for Wallpaper<WebSource<S>>
where
    WebSource<S>: WallpaperSource,
{
    type Source = MemorySource<WebSource<S>>;

    async fn set_wallpaper(self) -> Result<Wallpaper<Self::Source>, WallpaperError> {
        self.fetch().await?.set_wallpaper().await
    }
}
