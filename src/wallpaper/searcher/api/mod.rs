use super::SearchQuery;
use crate::wallpaper::{Wallpaper, error::WallpaperError, source::WebSource};

#[allow(async_fn_in_trait)]
pub trait WallpaperApi
where
    Self: Sized,
{
    async fn search(
        &self,
        query: &SearchQuery<Self>,
    ) -> Result<Vec<Wallpaper<WebSource<Self>>>, WallpaperError>;
}

pub mod wallhaven;
