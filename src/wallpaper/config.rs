use super::error::WallpaperError;
use configfs::{Config, ConfigFile};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Deserialize, Serialize, Default, Config)]
#[config(system = "wallpaper_app/settings_v2.toml")]
pub struct WallpaperConfig {
    pub searcher: SearcherConfig,
    pub setter: SetterConfig,

    pub wallpaper: Option<WallpaperPath>,
}

#[derive(Serialize, Deserialize, Clone)]
pub enum WallpaperPath {
    #[serde(rename = "url")]
    Url(String),
    #[serde(rename = "path")]
    Path(PathBuf),
}

#[derive(Deserialize, Serialize, Default, Debug)]
#[allow(dead_code)]
pub struct SearcherConfig {}

#[derive(Deserialize, Serialize, Debug)]
#[allow(dead_code)]
pub struct SetterConfig {
    /// Allow for a custom wallpaper path
    pub wallpapers_path: Option<PathBuf>,
    // NOTE: this is required
}

impl Default for SetterConfig {
    fn default() -> Self {
        let pictures_dir = dirs::picture_dir();
        let wallpaper_dir = pictures_dir.map(|pics| pics.join("wallpapers"));

        SetterConfig {
            wallpapers_path: wallpaper_dir,
        }
    }
}

impl WallpaperConfig {
    pub fn setup() -> Result<(), WallpaperError> {
        WallpaperConfig::init_or_default()?;

        let wall_config = WallpaperConfig::read()?;

        let WallpaperConfig {
            searcher: _,
            setter,
            ..
        } = &wall_config;

        if let Some(path) = &setter.wallpapers_path
            && !path.exists()
        {
            println!("Creating default wallpaper directory");

            std::fs::create_dir_all(path)?;
        }

        Ok(())
    }
}
