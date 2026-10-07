mod wallpaper;

use configfs::ConfigFile;
use rusqlite::Connection;
pub use wallpaper::searcher::query::SearchMode;

pub use wallpaper::db;
use wallpaper::{
    Wallpaper,
    error::WallpaperError,
    searcher::{SearchRatio, WebSearcher, api::Wallhaven, query::SearchQuery},
};

fn init_db() -> Result<Connection, WallpaperError> {
    // TODO: saving database file in .config file is not optimal, save into .local/share instead
    let dir = WallpaperConfig::config_directory();
    let (path, is_dir) = dir.resolve()?;

    let db_file = if is_dir {
        path.join("wallpapers.db")
    } else {
        path.parent().unwrap().join("wallpapers.db")
    };

    db::init(&db_file).map_err(WallpaperError::SqliteError)
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
