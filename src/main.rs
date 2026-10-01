use wallpaper::{WallpaperSetter, WallpaperSource};

#[tokio::main]
async fn main() -> Result<(), wallpaper::Error> {
    println!("Searching random wallpapers");
    let wallpapers =
        wallpaper::search_wallpapers(wallpaper::SearchMode::Query("itachi".into())).await?;

    if let Some(wallpaper) = wallpapers.first() {
        wallpaper.set_wallpaper().await?;
        println!(
            "set wallpaper {}: {}",
            wallpaper.source.id, wallpaper.source.image_url
        );

        let information = wallpaper.load_details().await?;

        dbg!(information);
    }

    Ok(())
}
