#[tokio::main]
async fn main() -> Result<(), wallpaper::Error> {
    println!("Searching random wallpapers");
    wallpaper::get_wallpapers(wallpaper::SearchMode::Random).await
}
