use image::ImageFormat;
use serde::{Deserialize, Serialize};

pub mod config;
pub mod db;
pub mod error;
pub mod searcher;
pub mod setter;
pub mod source;

#[allow(dead_code)]
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct WallpaperDetails {
    // NOTE: still not sure what details i should be storing as metadata
    pub title: Option<String>,
    pub author: Option<String>,
    pub created_at: Option<String>,
    pub tags: Vec<String>,

    pub url: String,
    pub resolution: String,
    pub ratio: String,
    pub file_size: u64,
    pub file_type: String,
}

#[allow(dead_code)]
pub struct Wallpaper<S> {
    pub image: WallpaperImage,
    pub tags: Vec<String>,
    pub source: S,
}

#[allow(dead_code)]
#[derive(Clone)]
pub struct WallpaperImage {
    pub ratio: f64,
    pub format: Option<ImageFormat>,
    pub name: String,
    pub metadata: Option<WallpaperDetails>,
}
