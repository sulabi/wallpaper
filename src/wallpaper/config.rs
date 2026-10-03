use super::error::WallpaperError;
use configfs::{Config, ConfigDirectory};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Deserialize, Serialize)]
#[allow(dead_code)]
pub struct WallpaperConfig {
    pub searcher: SearcherConfig,
    pub setter: SetterConfig,
}

#[derive(Deserialize, Serialize)]
#[allow(dead_code)]
pub struct SearcherConfig {}

#[derive(Deserialize, Serialize)]
#[allow(dead_code)]
pub struct SetterConfig {
    /// Allow for a custom wallpaper path
    pub wallpaper_path: Option<PathBuf>,

    /// Allow the user to have their own sub folders, named after category of the wallpapers
    /// included
    pub wallpaper_categories: Option<Vec<String>>,
}

impl Default for WallpaperConfig {
    fn default() -> Self {
        let pictures_dir = dirs::picture_dir();
        let wallpaper_dir = pictures_dir.map(|pics| pics.join("wallpapers"));

        WallpaperConfig {
            searcher: SearcherConfig {},
            setter: SetterConfig {
                wallpaper_path: wallpaper_dir,
                wallpaper_categories: Some(vec!["anime".into()]),
            },
        }
    }
}

impl WallpaperConfig {
    pub fn init() -> Result<WallpaperConfig, WallpaperError> {
        let configfs: Config<WallpaperConfig> =
            Config::new(ConfigDirectory::System("wallpaper_app/settings_v2.toml"))?;

        let wall_config = configfs.read_or_default()?;

        let WallpaperConfig {
            searcher: _,
            setter,
        } = &wall_config;

        if let Some(path) = &setter.wallpaper_path
            && !path.exists()
        {
            println!("Creating default wallpaper directory");

            std::fs::create_dir_all(path)?;
        }

        Ok(wall_config)
    }
}
