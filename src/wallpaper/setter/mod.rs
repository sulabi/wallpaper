use crate::{WallpaperSource, source::Fetch};

use super::{
    Wallpaper,
    error::WallpaperError,
    source::{MemorySource, WebSource},
};

#[cfg(target_os = "linux")]
mod linux;

#[allow(dead_code)]
#[allow(async_fn_in_trait)]
pub trait WallpaperSetter {
    type Source;
    async fn set_wallpaper(self) -> Result<Wallpaper<Self::Source>, WallpaperError>;
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
