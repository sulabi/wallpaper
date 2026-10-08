mod wallpaper;

use rusqlite::Connection;
pub use wallpaper::searcher::query::SearchMode;

pub use wallpaper::db;
use wallpaper::{
    Wallpaper,
    error::WallpaperError,
    searcher::{SearchRatio, WebSearcher, api::Wallhaven, query::SearchQuery},
};

fn init_db() -> Result<Connection, WallpaperError> {
    if let Some(data_dir) = dirs::data_dir() {
        let folder = data_dir.join("wallpaper_app");
        if !folder.exists() {
            std::fs::create_dir(&folder)?;
        }
        let path = folder.join("wallpapers.db");

        db::init(&path).map_err(WallpaperError::SqliteError)
    } else {
        Err(WallpaperError::DatabaseCreationError)
    }
}

fn init_conf() -> Result<(), WallpaperError> {
    WallpaperConfig::setup()
}

pub fn init() -> Result<Connection, WallpaperError> {
    init_conf()?;

    init_db()
}

pub async fn search_wallpapers(
    mode: SearchMode,
) -> Result<Vec<Wallpaper<WebSource<Wallhaven>>>, WallpaperError> {
    let searcher = WebSearcher::new(Wallhaven);
    let query = SearchQuery::new(mode, SearchRatio::All, "tag1");

    let wallpapers = searcher.search(&query).await?;

    Ok(wallpapers)
}

pub use wallpaper::error::WallpaperError as Error;
pub use wallpaper::setter::WallpaperSetter;
pub use wallpaper::source::WallpaperSource;

use crate::wallpaper::{config::WallpaperConfig, source::WebSource};
