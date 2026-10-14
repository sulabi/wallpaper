use rusqlite::Connection;
use std::io::Write;
use wallpaper::{
    Save, SearchMode, SearchParams, Wallpaper, WallpaperApi, WallpaperSetter, api, db,
    source::{AsWallpaperPath, Fetch},
};

fn prompt(msg: &str) -> std::io::Result<String> {
    print!("{msg}");
    std::io::stdout().flush()?;
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf)?;
    Ok(buf.trim().to_owned())
}

#[tokio::main]
async fn main() -> Result<(), wallpaper::Error> {
    let connection = wallpaper::init()?;

    let query = prompt("enter query: ")?;
    let source = prompt("web or local: ")?;

    match source.as_str() {
        "web" => search_set(api::Wallhaven, query, &connection).await,
        "local" => {
            search_set(
                api::Local {
                    connection: &connection,
                },
                query,
                &connection,
            )
            .await
        }
        other => {
            eprintln!("unknown source: {other}");
            Ok(())
        }
    }
}

// TODO: integrate with clap
// commands i want: populate (download many), interactive mode (view, favourite, download, more..)

async fn search_set<T>(api: T, query: String, conn: &Connection) -> Result<(), wallpaper::Error>
where
    T: WallpaperApi,
    Wallpaper<T::Source>: Fetch<Origin = T::Source>,
    T::Source: AsWallpaperPath + Save,
{
    let params = SearchParams::new(SearchMode::Query(query), api);

    let wallpapers = wallpaper::search_wallpapers(params).await?;

    let Some(wallpaper) = wallpapers.into_iter().next() else {
        println!("no wallpapers found");
        return Ok(());
    };

    println!("preparing wallpaper...");
    let mem = wallpaper.fetch().await?;
    println!("wallpaper: {:?}", mem.image.name);
    let mem = mem.set_wallpaper().await?;

    let local = mem.save().await?;
    println!("saving to db...");
    db::add_wallpaper(conn, local)?;
    Ok(())
}
