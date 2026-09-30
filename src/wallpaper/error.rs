use thiserror::Error;

#[derive(Error, Debug)]
pub enum WallpaperError {
    #[error("Search failed")]
    SearchError(#[from] reqwest::Error),
}
