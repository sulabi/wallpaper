pub mod error;
pub mod searcher;
pub mod setter;
pub mod source;

#[allow(dead_code)]
pub struct WallpaperDetails {
    pub title: Option<String>,
    pub author: Option<String>,
}

#[allow(dead_code)]
pub struct Wallpaper<S> {
    pub id: String,
    pub image: WallpaperImage,
    pub source: S,
}

#[allow(dead_code)]
pub struct WallpaperImage {
    pub ratio: f64,
}
