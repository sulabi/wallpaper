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

    fn round2(val: f64) -> f64 {
        (val * 100.).round() / 100.
    }

    #[tokio::test]
    async fn search_portrait_ratio() -> Result<(), WallpaperError> {
        let searcher = WebSearcher::new(Wallhaven);

        let query = SearchQuery::new(
            SearchMode::Random,
            SearchRatio::Portrait(PortraitRatio::All),
        );

        let wallpapers = searcher.search(&query).await?;
        let wallpaper = wallpapers.first().expect("No wallpapers received");

        assert!(
            [9. / 16., 9. / 18., 10. / 16.]
                .iter()
                .any(|&ratio| round2(ratio) == wallpaper.image.ratio),
            "Ratio doesnt match portrait"
        );

        Ok(())
    }
}
