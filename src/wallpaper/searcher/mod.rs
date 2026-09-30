use crate::wallpaper::Wallpaper;

use super::error::WallpaperError;
use super::source::{LocalSource, WebSource};

mod api;
use api::WallpaperApi;

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
        query: &SearchQuery,
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

#[derive(Debug)]
pub struct SearchQuery {
    pub mode: SearchMode,
}

#[derive(Debug)]
pub enum SearchMode {
    Query(String),
    Random,
}

#[cfg(test)]
mod tests {
    use super::*;
    use api::wallhaven::Wallhaven;

    #[tokio::test]
    async fn search_wallhaven() -> Result<(), WallpaperError> {
        let searcher = WebSearcher::new(Wallhaven);

        let query = SearchQuery {
            mode: SearchMode::Query("mountain".into()),
        };

        let wallpapers = searcher.search(&query).await?;

        for wallpaper in &wallpapers {
            println!("wallpaper ({}): {}", wallpaper.id, wallpaper.source.url);
        }

        Ok(())
    }
}
