use crate::wallpaper::{Wallpaper, WallpaperDetails};

use super::error::WallpaperError;

pub struct WebSource<A> {
    pub url: String,
    pub api: A,
}

pub struct LocalSource {
    pub path: std::path::PathBuf,
}

#[allow(async_fn_in_trait)]
pub trait WallpaperSource {
    async fn load_wallpaper(&self) -> Result<WallpaperDetails, WallpaperError>;
}

impl<A> Wallpaper<WebSource<A>>
where
    WebSource<A>: WallpaperSource,
{
    pub async fn download(self) -> Result<Wallpaper<LocalSource>, WallpaperError> {
        let _img = self.image;
        todo!("download and convert to localwallpaper")
    }
}

impl Wallpaper<LocalSource> {
    pub fn delete(self) -> Result<(), WallpaperError> {
        todo!("delete and nothing")
    }
}
