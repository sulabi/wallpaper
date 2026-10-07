use wallpaper::{WallpaperSetter, WallpaperSource, db};

#[tokio::main]
async fn main() -> Result<(), wallpaper::Error> {
    println!("Searching random wallpapers");
    let connection = wallpaper::init()?;

    let wallpapers =
        wallpaper::search_wallpapers(wallpaper::SearchMode::Query("itachi".into())).await?;

    if let Some(wallpaper) = wallpapers.into_iter().next() {
        let wallpaper = wallpaper.set_wallpaper().await?;
        let information = wallpaper.source.origin.load_details().await;
        println!("wallpaper source: {:?}", wallpaper.source);
        println!("wallpaper information: {:?}", information);

        println!("downloading wallpaper ...");
        let saved_wallpaper = wallpaper.save().await?;

        db::add_wallpaper(&connection, saved_wallpaper)?;
    }

    Ok(())
}
