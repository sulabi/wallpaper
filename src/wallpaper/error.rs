use thiserror::Error;

#[derive(Error, Debug)]
pub enum WallpaperError {
    #[error("Search failed: {0}")]
    SearchError(#[from] reqwest::Error),

    #[error("I/O failed: {0}")]
    IOError(#[from] std::io::Error),

    #[error("awww exited with status: {0}")]
    SetterError(std::process::ExitStatus),
}
