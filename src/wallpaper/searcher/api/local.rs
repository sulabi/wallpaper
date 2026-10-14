use crate::{
    WallpaperSource,
    wallpaper::{
        Wallpaper, WallpaperDetails,
        error::WallpaperError,
        searcher::{SearchQuery, api::WallpaperApi},
        source::LocalSource,
    },
};

#[derive(Debug)]
pub struct Local;

impl WallpaperApi for Local {
    type Source = LocalSource;

    async fn search(
        &self,
        _query: &SearchQuery<Self>,
    ) -> Result<Vec<Wallpaper<Self::Source>>, WallpaperError> {
        todo!()
    }
}

impl WallpaperSource for Local {
    async fn load_details(&self) -> Result<WallpaperDetails, WallpaperError> {
        todo!()
    }
}
