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

#[allow(async_fn_in_trait)]
pub trait Save: Sized {
    async fn save(
        mem_src: Wallpaper<MemorySource<Self>>,
    ) -> Result<Wallpaper<LocalSource>, WallpaperError>;
}

#[allow(async_fn_in_trait)]
pub trait Fetch {
    type Origin: Save;
    async fn fetch(self) -> Result<Wallpaper<MemorySource<Self::Origin>>, WallpaperError>;
}

#[allow(async_fn_in_trait)]
pub trait Download: Fetch + Sized {
    async fn download(self) -> Result<Wallpaper<LocalSource>, WallpaperError> {
        self.fetch().await?.save().await
    }
}
impl<T: Fetch> Download for T {}

impl<S: Save> Wallpaper<MemorySource<S>> {
    pub async fn save(self) -> Result<Wallpaper<LocalSource>, WallpaperError> {
        S::save(self).await
    }
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
impl WallpaperSource for LocalSource {
    async fn load_details(&self) -> Result<WallpaperDetails, WallpaperError> {
        todo!()
    }
}

// web fetch + save

impl<A> Fetch for Wallpaper<WebSource<A>>
where
    WebSource<A>: WallpaperSource,
{
    type Origin = WebSource<A>;

    async fn fetch(self) -> Result<Wallpaper<MemorySource<WebSource<A>>>, WallpaperError> {
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

impl<A> Save for WebSource<A> {
    async fn save(
        mem_src: Wallpaper<MemorySource<Self>>,
    ) -> Result<Wallpaper<LocalSource>, WallpaperError> {
        let conf = WallpaperConfig::get();
        let wallpapers_folder = conf
            .setter
            .wallpapers_path
            .as_ref()
            .ok_or(WallpaperError::NoWallpapersFolder)?;

        std::fs::create_dir_all(wallpapers_folder)?;

        let mut path = wallpapers_folder.join(&mem_src.image.name);
        if let Some(format) = mem_src.image.format {
            path.set_extension(format.extensions_str()[0]);
        }
        std::fs::write(&path, &mem_src.source.bytes)?;

        Ok(Wallpaper {
            image: mem_src.image,
            tags: mem_src.tags,
            source: LocalSource { path },
        })
    }
}

// local fetch + save(ignored)

impl Fetch for Wallpaper<LocalSource> {
    type Origin = LocalSource;

    async fn fetch(mut self) -> Result<Wallpaper<MemorySource<LocalSource>>, WallpaperError> {
        let bytes = std::fs::read(&self.source.path)?;
        if self.image.format.is_none() {
            self.image.format = image::guess_format(&bytes).ok();
        }

        Ok(Wallpaper {
            image: self.image,
            tags: self.tags,
            source: MemorySource {
                origin: self.source,
                bytes,
            },
        })
    }
}

impl Save for LocalSource {
    async fn save(
        mem_src: Wallpaper<MemorySource<Self>>,
    ) -> Result<Wallpaper<LocalSource>, WallpaperError> {
        Ok(Wallpaper {
            image: mem_src.image,
            tags: mem_src.tags,
            source: mem_src.source.origin,
        })
    }
}
