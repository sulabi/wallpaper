use std::path::PathBuf;

use crate::wallpaper::{Wallpaper, WallpaperDetails};

use super::error::WallpaperError;

#[allow(dead_code)]
pub struct WebSource<A> {
    pub id: String,
    pub image_url: String,
    pub api: A,
}

#[allow(dead_code)]
pub struct LocalSource {
    pub path: std::path::PathBuf,
}

pub struct MemorySource {
    pub id: String,
    pub bytes: Vec<u8>,
    // NOTE: TEMP
    pub image_url: String,
}

#[allow(async_fn_in_trait, dead_code)]
pub trait WallpaperSource {
    async fn load_details(&self) -> Result<WallpaperDetails, WallpaperError>;
}

impl<A> WallpaperSource for Wallpaper<WebSource<A>>
where
    WebSource<A>: WallpaperSource,
{
    async fn load_details(&self) -> Result<WallpaperDetails, WallpaperError> {
        self.source.load_details().await
    }
}

#[allow(dead_code)]
impl<A> Wallpaper<WebSource<A>> {
    pub async fn fetch(&self) -> Result<Wallpaper<MemorySource>, WallpaperError> {
        let bytes = reqwest::get(&self.source.image_url)
            .await?
            .bytes()
            .await?
            .to_vec();

        Ok(Wallpaper {
            image: self.image,
            source: MemorySource {
                bytes,
                id: self.source.id.clone(),
                image_url: self.source.image_url.clone(),
            },
        })
    }
}

impl Wallpaper<MemorySource> {
    pub async fn download() -> Result<PathBuf, WallpaperError> {
        todo!()
    }
}
