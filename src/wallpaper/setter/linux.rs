use super::{MemorySource, Wallpaper, WallpaperError, WallpaperSetter};
use crate::wallpaper::{
    config::{WallpaperConfig, WallpaperPath},
    source::{AsWallpaperPath, LocalSource},
};
use configfs::ConfigFile;
use std::path::Path;

impl<S: AsWallpaperPath> WallpaperSetter for Wallpaper<MemorySource<S>> {
    type Source = MemorySource<S>;

    async fn set_wallpaper(self) -> Result<Wallpaper<Self::Source>, WallpaperError> {
        let tmpfile = tempfile::NamedTempFile::new()?;
        std::fs::write(tmpfile.path(), &self.source.bytes)?;

        set_path(tmpfile.path(), self.source.origin.as_wallpaper_path())?;

        Ok(self)
    }
}

impl WallpaperSetter for Wallpaper<LocalSource> {
    type Source = LocalSource;

    async fn set_wallpaper(self) -> Result<Wallpaper<Self::Source>, WallpaperError> {
        set_path(&self.source.path, self.source.as_wallpaper_path())?;
        Ok(self)
    }
}

fn set_path(path: &Path, configured_path: WallpaperPath) -> Result<(), WallpaperError> {
    println!("setting {}", path.display());

    // TODO: make this a wrapper and configurable for other OS
    // TODO: also allow other args
    let status = std::process::Command::new("awww")
        .arg("img")
        .arg(path)
        .status()?;

    if !status.success() {
        return Err(WallpaperError::SetterError(status));
    }

    WallpaperConfig::update(|conf| {
        conf.wallpaper = Some(configured_path);
    })?;

    Ok(())
}
