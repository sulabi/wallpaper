pub mod error;
pub mod searcher;
pub mod setter;
pub mod source;

pub struct Wallpaper<S> {
    pub title: Option<String>,
    pub author: Option<String>,
    pub image: WallpaperImage,

    pub source: S,
}

pub struct WallpaperImage {}
