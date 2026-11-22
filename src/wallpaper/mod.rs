use std::path::PathBuf;

use clap::builder::OsStr;
use image::ImageFormat;
use serde::{Deserialize, Serialize};

use crate::wallpaper::error::WallpaperError;

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
pub struct WallpaperImage {
    pub ratio: f64,
    pub format: Option<ImageFormat>,
    pub name: String,
    pub metadata: Option<WallpaperDetails>,
}

impl Default for WallpaperImage {
    fn default() -> Self {
        WallpaperImage {
            ratio: 0_f64,
            format: None,
            name: "".to_string(),
            metadata: None,
        }
    }
}

impl WallpaperImage {
    pub fn from_path(path: &PathBuf) -> Result<WallpaperImage, WallpaperError> {
        let bytes = std::fs::read(path)?;
        let format = image::guess_format(&bytes).ok();
        let file_name = path
            .file_name()
            .unwrap_or(&OsStr::from(""))
            .to_string_lossy()
            .to_string();

        Ok(WallpaperImage {
            format,
            name: file_name,
            metadata: None,
            ratio: 0f64,
        })
    }
}
