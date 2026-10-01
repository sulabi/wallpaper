pub mod error;
pub mod searcher;
pub mod setter;
pub mod source;

#[allow(dead_code)]
#[derive(Debug)]
pub struct WallpaperDetails {
    pub title: Option<String>,
    pub author: Option<String>,
    pub category: Option<String>,
    pub created_at: Option<String>,

    pub url: String,
    pub resolution: String,
    pub ratio: String,
    pub file_size: u64,
    pub file_type: String,
}

#[allow(dead_code)]
pub struct Wallpaper<S> {
    pub image: WallpaperImage,
    pub source: S,
}

#[allow(dead_code)]
#[derive(Clone, Copy)]
pub struct WallpaperImage {
    pub ratio: f64,
}
