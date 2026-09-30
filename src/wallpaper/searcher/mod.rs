use super::Wallpaper;
use super::error::WallpaperError;
use super::source::{LocalSource, WebSource};

pub struct WebSearcher {}
pub struct LocalSearcher {}

impl Default for WebSearcher {
    fn default() -> Self {
        Self::new()
    }
}

impl WebSearcher {
    pub fn new() -> Self {
        Self {}
    }

    pub async fn search(
        &self,
        query: &SearchQuery,
    ) -> Result<Vec<Wallpaper<WebSource>>, WallpaperError> {
        match &query.mode {
            SearchMode::Random => println!("searching random!"),
            SearchMode::Query(query) => println!("searching: `{}`", query),
        }

        todo!()
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

    #[tokio::test]
    async fn create_search() {
        let searcher = WebSearcher::new();

        let query = SearchQuery {
            mode: SearchMode::Random,
        };

        let results = searcher.search(&query).await;
    }
}
