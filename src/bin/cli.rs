use clap::{Args, Parser, Subcommand, ValueEnum};
use std::process::ExitCode;
use wallpaper::{
    Connection, Save, SearchMode, SearchParams, Wallpaper, WallpaperApi, WallpaperSetter, api, db,
    source::{AsWallpaperPath, Fetch},
};

#[derive(Parser)]
#[command(
    name = "wallpaper",
    version,
    about = "Search, download, and set wallpapers"
)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// List matching wallpapers without downloading or setting them
    Search(SearchArgs),
    /// Set the first matching wallpaper and save web downloads to the library
    Set(SearchArgs),

    /// Download matching Wallhaven wallpapers into the local library
    Populate {
        /// Search terms; omit to request random wallpapers
        query: Option<String>,
        /// Maximum number to download from the returned search results
        #[arg(short, long, default_value = "10", value_parser = clap::value_parser!(u32).range(1..))]
        count: u32,
    },
}

#[derive(Args)]
struct SearchArgs {
    /// Search terms; omit to request random wallpapers
    query: Option<String>,
    /// Search Wallhaven or the saved local library
    #[arg(short, long, value_enum, default_value_t = Source::Web)]
    source: Source,
}

#[derive(Clone, Copy, ValueEnum)]
enum Source {
    Web,
    Local,
}

fn search_mode(query: Option<String>) -> SearchMode {
    match query {
        Some(query) => SearchMode::Query(query),
        None => SearchMode::Random,
    }
}

#[tokio::main]
async fn main() -> ExitCode {
    let cli = Cli::parse();
    match run(cli).await {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("error: {error}");
            ExitCode::FAILURE
        }
    }
}

async fn run(cli: Cli) -> Result<(), wallpaper::Error> {
    let connection = wallpaper::init()?;
    match cli.command {
        Command::Search(args) => match args.source {
            Source::Web => search(api::Wallhaven, args.query, &connection, false, false).await,
            Source::Local => {
                search(
                    api::Local::new(&connection),
                    args.query,
                    &connection,
                    false,
                    false,
                )
                .await
            }
        },
        Command::Set(args) => match args.source {
            Source::Web => search(api::Wallhaven, args.query, &connection, true, true).await,
            Source::Local => {
                search(
                    api::Local::new(&connection),
                    args.query,
                    &connection,
                    true,
                    false,
                )
                .await
            }
        },
        Command::Populate { query, count } => {
            let params = SearchParams::new(search_mode(query), api::Wallhaven);
            let wallpapers = wallpaper::search_wallpapers(params).await?;
            let mut saved = 0;
            for wallpaper in wallpapers.into_iter().take(count as usize) {
                let local = wallpaper.fetch().await?.save().await?;
                println!("Saved {}", local.source.path.display());
                db::add_wallpaper(&connection, local)?;
                saved += 1;
            }
            println!("Saved {saved} wallpaper(s).");
            Ok(())
        }
    }
}

async fn search<T>(
    api: T,
    query: Option<String>,
    conn: &Connection,
    set: bool,
    register: bool,
) -> Result<(), wallpaper::Error>
where
    T: WallpaperApi,
    Wallpaper<T::Source>: Fetch<Origin = T::Source>,
    T::Source: AsWallpaperPath + Save,
{
    let params = SearchParams::new(search_mode(query), api);
    let wallpapers = wallpaper::search_wallpapers(params).await?;
    if wallpapers.is_empty() {
        println!("No wallpapers found.");
        return Ok(());
    }

    if !set {
        for wallpaper in wallpapers {
            let location = match wallpaper.source.as_wallpaper_path() {
                wallpaper::config::WallpaperPath::Url(url) => url,
                wallpaper::config::WallpaperPath::Path(path) => path.display().to_string(),
            };
            println!("{}\t{location}", wallpaper.image.name);
        }
        return Ok(());
    }

    if let Some(wallpaper) = wallpapers.into_iter().next() {
        // TODO: fix wallpapers from local going to temp
        let mem = wallpaper.fetch().await?;
        println!("Setting {}", mem.image.name);
        let mem = mem.set_wallpaper().await?;
        if register {
            let local = mem.save().await?;
            db::add_wallpaper(conn, local)?;
        }
    }
    Ok(())
}
