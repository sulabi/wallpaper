use super::SearchQuery;
use crate::wallpaper::{Wallpaper, error::WallpaperError};

#[allow(async_fn_in_trait)]
pub trait WallpaperApi
where
    Self: Sized,
{
    type Source;

    async fn search(
        &self,
        query: &SearchQuery<Self>,
    ) -> Result<Vec<Wallpaper<Self::Source>>, WallpaperError>;
}

pub mod local;
pub mod wallhaven;
pub use local::Local;
pub use wallhaven::Wallhaven;
