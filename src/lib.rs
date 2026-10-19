mod wallpaper;

pub use rusqlite::Connection;
pub use wallpaper::{db, searcher::query::SearchMode};
use wallpaper::{
    error::WallpaperError,
    searcher::{SearchRatio, Searcher, query::SearchQuery},
};

fn init_db() -> Result<Connection, WallpaperError> {
    let data_dir = dirs::data_dir().ok_or(WallpaperError::DatabaseCreationError)?;

    let folder = data_dir.join("wallpaper_app");
    std::fs::create_dir_all(&folder)?;

    db::init(&folder.join("wallpapers.db")).map_err(WallpaperError::SqliteError)
}

fn init_conf() -> Result<(), WallpaperError> {
    WallpaperConfig::setup()
}

pub fn init() -> Result<Connection, WallpaperError> {
    init_conf()?;

    init_db()
}

pub async fn search_wallpapers<T: WallpaperApi>(
    params: SearchParams<T>,
) -> Result<Vec<Wallpaper<T::Source>>, WallpaperError> {
    let searcher = Searcher::new(params.search_api);
    let query = SearchQuery::new(params.search_mode, SearchRatio::All);

    searcher.search(&query).await
}

pub use wallpaper::error::WallpaperError as Error;
pub use wallpaper::{
    setter::WallpaperSetter,
    source::{Save, WallpaperSource},
};

pub use crate::wallpaper::searcher::api;
pub use crate::wallpaper::{
    Wallpaper, config,
    config::WallpaperConfig,
    searcher,
    searcher::{api::WallpaperApi, query::SearchParams},
    setter, source,
};
