use rusqlite::Connection;

use crate::{
    WallpaperSource, db,
    wallpaper::{
        Wallpaper, WallpaperDetails,
        error::WallpaperError,
        searcher::{SearchQuery, api::WallpaperApi},
        source::LocalSource,
    },
};

#[derive(Debug)]
pub struct Local<'a> {
    pub connection: &'a Connection,
}

impl WallpaperApi for Local<'_> {
    type Source = LocalSource;

    async fn search(
        &self,
        query: &SearchQuery<Self>,
    ) -> Result<Vec<Wallpaper<Self::Source>>, WallpaperError> {
        db::search_wallpaper(self.connection, &query.tags, 10).map_err(WallpaperError::SqliteError)
    }
}

impl WallpaperSource for Local<'_> {
    async fn load_details(&self) -> Result<WallpaperDetails, WallpaperError> {
        todo!()
    }
}
