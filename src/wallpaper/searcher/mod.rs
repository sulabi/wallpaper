use super::{Wallpaper, error::WallpaperError};

pub mod api;
pub mod query;

use api::WallpaperApi;
pub use query::{SearchQuery, SearchRatio};

#[allow(dead_code)]
pub struct Searcher<A> {
    pub api: A,
}

impl<T: WallpaperApi> Searcher<T> {
    pub fn new(api: T) -> Self {
        Self { api }
    }

    pub async fn search(
        &self,
        query: &SearchQuery<T>,
    ) -> Result<Vec<Wallpaper<T::Source>>, WallpaperError> {
        self.api.search(query).await
    }
}

#[cfg(test)]
mod tests {
    use super::query::{PortraitRatio, SearchMode, SearchQuery, SearchRatio};
    use super::*;
    use api::wallhaven::Wallhaven;

    fn round2(val: f64) -> f64 {
        (val * 100.).round() / 100.
    }

    #[tokio::test]
    async fn search_portrait_ratio() -> Result<(), WallpaperError> {
        let searcher = Searcher::new(Wallhaven);

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
