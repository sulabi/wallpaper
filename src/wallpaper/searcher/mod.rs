use super::{
    Wallpaper,
    error::WallpaperError,
    source::{LocalSource, WebSource},
};

mod api;
mod query;

use api::WallpaperApi;
pub use query::{PortraitRatio, Ratio, SearchMode, SearchQuery, SquareRatio, WideRatio};

#[derive(Default)]
pub struct WebSearcher<Api> {
    pub api: Api,
}

pub struct LocalSearcher {}

impl<T: WallpaperApi> WebSearcher<T> {
    pub fn new(api: T) -> Self {
        Self { api }
    }

    pub async fn search(
        &self,
        query: &SearchQuery<T>,
    ) -> Result<Vec<Wallpaper<WebSource<T>>>, WallpaperError> {
        match &query.mode {
            SearchMode::Random => println!("searching random!"),
            SearchMode::Query(query) => println!("searching: `{}`", query),
        }

        self.api.search(query).await
    }
}

impl LocalSearcher {
    pub async fn search(&self) -> Vec<Wallpaper<LocalSource>> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::query::{PortraitRatio, SearchQuery, SearchRatio};
    use super::*;
    use api::wallhaven::Wallhaven;

    #[tokio::test]
    async fn search_wallhaven() -> Result<(), WallpaperError> {
        let searcher = WebSearcher::new(Wallhaven);

        let query: SearchQuery<Wallhaven> =
            SearchQuery::new(SearchMode::Query("mountain".into()), SearchRatio::All);

        let wallpapers = searcher.search(&query).await?;

        for wallpaper in &wallpapers {
            println!("wallpaper ({}): {}", wallpaper.id, wallpaper.source.url);
        }

        Ok(())
    }

    #[tokio::test]
    async fn search_portrait_ratio() -> Result<(), WallpaperError> {
        let searcher = WebSearcher::new(Wallhaven);

        let query: SearchQuery<Wallhaven> = SearchQuery::new(
            SearchMode::Random,
            SearchRatio::Portrait(PortraitRatio::All),
        );

        let wallpapers = searcher.search(&query).await?;

        if let Some(wallpaper) = wallpapers.first() {
            println!(
                "wallpaper portrait ratio ({}): {}",
                wallpaper.id, wallpaper.source.url
            );
        }

        Ok(())
    }
}
