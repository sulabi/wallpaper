mod wallpaper;

pub use wallpaper::searcher::query::SearchMode;

use wallpaper::{
    Wallpaper,
    error::WallpaperError,
    searcher::{SearchRatio, WebSearcher, api::Wallhaven, query::SearchQuery},
};

pub fn init_conf() -> Result<(), WallpaperError> {
    WallpaperConfig::setup()
}

pub async fn search_wallpapers(
    mode: SearchMode,
) -> Result<Vec<Wallpaper<WebSource<Wallhaven>>>, WallpaperError> {
    let searcher = WebSearcher::new(Wallhaven);
    let query = SearchQuery::new(mode, SearchRatio::All);

    let wallpapers = searcher.search(&query).await?;

    Ok(wallpapers)
}

pub use wallpaper::error::WallpaperError as Error;
pub use wallpaper::setter::WallpaperSetter;
pub use wallpaper::source::WallpaperSource;

use crate::wallpaper::{config::WallpaperConfig, source::WebSource};
