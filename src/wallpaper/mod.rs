pub mod error;
pub mod searcher;
pub mod setter;
pub mod source;

pub struct WallpaperDetails {
    pub title: Option<String>,
    pub author: Option<String>,
}

pub struct Wallpaper<S> {
    pub id: String,
    pub image: WallpaperImage,
    pub source: S,
}

pub struct WallpaperImage {
    pub ratio: f64,
}
