use super::{MemorySource, Wallpaper, WallpaperError, WallpaperSetter};
use crate::wallpaper::{config::WallpaperConfig, source::AsWallpaperPath};
use configfs::ConfigFile;

impl<S> WallpaperSetter for Wallpaper<MemorySource<S>>
where
    S: AsWallpaperPath,
{
    type Source = MemorySource<S>;

    async fn set_wallpaper(self) -> Result<Wallpaper<Self::Source>, WallpaperError> {
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

        let path = self.source.origin.as_wallpaper_path();

        WallpaperConfig::update(|conf| {
            conf.wallpaper = Some(path);
        })?;

        Ok(self)
    }
}
