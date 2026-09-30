mod wallpaper;

pub use wallpaper::searcher::query::SearchMode;

use wallpaper::{
    error::WallpaperError,
    searcher::{SearchRatio, WebSearcher, api::Wallhaven, query::SearchQuery},
};

pub async fn get_wallpapers(mode: SearchMode) -> Result<(), WallpaperError> {
    let searcher = WebSearcher::new(Wallhaven);
    let query = SearchQuery::new(mode, SearchRatio::All);

    let wallpapers = searcher.search(&query).await?;

    for wallpaper in wallpapers.iter() {
        println!("{}", wallpaper.source.url);
    }

    Ok(())
}

pub use wallpaper::error::WallpaperError as Error;
