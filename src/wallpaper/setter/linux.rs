use configfs::ConfigFile;

use crate::wallpaper::config::{WallpaperConfig, WallpaperPath};

use super::{MemorySource, Wallpaper, WallpaperError, WallpaperSetter};

impl WallpaperSetter for Wallpaper<MemorySource> {
    async fn set_wallpaper(&self) -> Result<(), WallpaperError> {
        let tmpfile = tempfile::NamedTempFile::new()?;
        std::fs::write(tmpfile.path(), &self.source.bytes)?;

        // TODO: make this a wrapper and configurable for other OS
        // TODO: also allow other args
        let status = std::process::Command::new("awww")
            .args(["img", tmpfile.path().to_str().unwrap()])
            .status()?;

        if !status.success() {
            return Err(WallpaperError::SetterError(status));
        }

        WallpaperConfig::update(|conf| {
            conf.wallpaper = Some(WallpaperPath::Url(self.source.image_url.clone()));
        })?;

        Ok(())
    }
}
