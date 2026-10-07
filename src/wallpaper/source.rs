use super::error::WallpaperError;
use crate::wallpaper::{
    Wallpaper, WallpaperDetails, WallpaperImage,
    config::{WallpaperConfig, WallpaperPath},
};
use configfs::ConfigFile;
use std::fmt::Debug;

#[allow(dead_code)]
#[derive(Debug)]
pub struct WebSource<A> {
    pub id: String,
    pub image_url: String,
    pub api: A,
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct LocalSource {
    pub path: std::path::PathBuf,
}

pub struct MemorySource<S> {
    pub origin: S,
    pub bytes: Vec<u8>,
}

impl<S: Debug> Debug for MemorySource<S> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MemorySource")
            .field("origin", &self.origin)
            .field("bytes", &format_args!("<{} bytes>", self.bytes.len()))
            .finish()
    }
}

#[allow(async_fn_in_trait, dead_code)]
pub trait WallpaperSource {
    async fn load_details(&self) -> Result<WallpaperDetails, WallpaperError>;
}

pub trait AsWallpaperPath {
    fn as_wallpaper_path(&self) -> WallpaperPath;
}

impl<A> AsWallpaperPath for WebSource<A> {
    fn as_wallpaper_path(&self) -> WallpaperPath {
        WallpaperPath::Url(self.image_url.clone())
    }
}

impl AsWallpaperPath for LocalSource {
    fn as_wallpaper_path(&self) -> WallpaperPath {
        WallpaperPath::Path(self.path.clone())
    }
}

impl<A> WallpaperSource for Wallpaper<WebSource<A>>
where
    WebSource<A>: WallpaperSource,
{
    async fn load_details(&self) -> Result<WallpaperDetails, WallpaperError> {
        self.source.load_details().await
    }
}

impl<A> WallpaperSource for MemorySource<WebSource<A>>
where
    WebSource<A>: WallpaperSource,
{
    async fn load_details(&self) -> Result<WallpaperDetails, WallpaperError> {
        self.origin.load_details().await
    }
}

#[allow(dead_code)]
impl<A> Wallpaper<WebSource<A>>
where
    WebSource<A>: WallpaperSource,
{
    pub async fn fetch(self) -> Result<Wallpaper<MemorySource<WebSource<A>>>, WallpaperError> {
        let bytes = reqwest::get(&self.source.image_url)
            .await?
            .bytes()
            .await?
            .to_vec();
        let other_details = self.load_details().await?;

        let format = image::guess_format(&bytes).ok();

        Ok(Wallpaper {
            image: WallpaperImage {
                ratio: self.image.ratio,
                name: format!("wallhaven-{}", self.source.id),
                format,
                metadata: Some(other_details.clone()),
            },
            source: MemorySource {
                origin: self.source,
                bytes,
            },
            tags: other_details.tags,
        })
    }
}

impl<A> Wallpaper<WebSource<A>>
where
    WebSource<A>: WallpaperSource,
{
    pub async fn download(self) -> Result<Wallpaper<LocalSource>, WallpaperError> {
        let fetched_wallpaper = self.fetch().await?;
        let other_data = fetched_wallpaper.source.origin.load_details().await?;
        let downloaded_wallpaper = fetched_wallpaper.save().await?;

        Ok(Wallpaper {
            image: fetched_wallpaper.image,
            tags: other_data.tags,
            source: downloaded_wallpaper.source,
        })
    }
}

impl<A> Wallpaper<MemorySource<WebSource<A>>> {
    pub async fn save(&self) -> Result<Wallpaper<LocalSource>, WallpaperError> {
        let conf = WallpaperConfig::get();
        let setter_conf = &conf.setter;

        let wallpapers_folder = setter_conf
            .wallpapers_path
            .as_ref()
            .ok_or(WallpaperError::NoWallpapersFolder)?;

        let filename = self.image.name.clone();
        let mut path = wallpapers_folder.join(filename);

        std::fs::create_dir_all(wallpapers_folder)?;

        if let Some(format) = self.image.format {
            path.set_extension(format.extensions_str()[0]);
        }
        std::fs::write(&path, &self.source.bytes)?;

        Ok(Wallpaper {
            image: self.image.clone(),
            source: LocalSource { path },
            tags: self.tags.clone(),
        })
    }
}
