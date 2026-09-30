use super::WallpaperApi;
use crate::wallpaper::{
    Wallpaper, WallpaperDetails, WallpaperImage,
    error::WallpaperError,
    searcher::{SearchMode, SearchQuery},
    source::{WallpaperSource, WebSource},
};
use serde::Deserialize;

#[derive(Default)]
pub struct Wallhaven;

#[derive(Debug, Deserialize)]
#[allow(unused)]
struct SearchResult {
    url: String,
    id: String,
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
struct SearchResponse {
    data: Vec<SearchResult>,
}

impl WallpaperApi for Wallhaven {
    async fn search(
        &self,
        query: &SearchQuery,
    ) -> Result<Vec<Wallpaper<WebSource<Self>>>, WallpaperError> {
        let url = format!(
            "https://wallhaven.cc/api/v1/search?q={}&sorting={}",
            match &query.mode {
                SearchMode::Query(q) => q,
                SearchMode::Random => "",
            },
            match &query.mode {
                SearchMode::Query(..) => "date_added",
                SearchMode::Random => "random",
            },
        );

        let res: SearchResponse = reqwest::get(url).await?.json().await?;
        let data = res.data;

        Ok(data.into_iter().map(Self::parse_preview).collect())
    }
}

impl WallpaperSource for WebSource<Wallhaven> {
    async fn load_wallpaper(&self) -> Result<WallpaperDetails, WallpaperError> {
        // request extra data using self.id
        todo!()
    }
}

impl Wallhaven {
    fn parse_preview(result: SearchResult) -> Wallpaper<WebSource<Self>> {
        Wallpaper {
            id: result.id,
            source: WebSource {
                url: result.url,
                api: Wallhaven,
            },
            image: WallpaperImage {},
        }
    }
}
