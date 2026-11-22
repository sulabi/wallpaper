use clap::{Args, Parser, Subcommand, ValueEnum};
use configfs::ConfigFile;
use std::process::ExitCode;
use wallpaper::{
    Connection, Save, SearchMode, SearchParams, Wallpaper, WallpaperApi, WallpaperConfig,
    WallpaperImage, WallpaperSetter, api,
    config::WallpaperPath,
    db,
    source::{AsWallpaperPath, Fetch, LocalSource},
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
    Set(SetArgs),

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
struct SetArgs {
    /// wallpaper search query; omit for random wallpapers
    query: Option<String>,
    /// search from wallhaven or from downloaded wallpapers
    #[arg(short, long, value_enum)]
    source: Option<Source>,

    /// picks a random wallpaper
    #[arg(short, long, default_value_t = false)]
    random: bool,

    #[arg(short, long, conflicts_with_all = ["random", "source", "query"], default_value_t = false)]
    from_config: bool,
}

#[derive(Args)]
struct SearchArgs {
    /// wallpaper search query; omit for random wallpapers
    query: Option<String>,
    /// search from wallhaven or from downloaded wallpapers
    #[arg(short, long, value_enum, default_value_t = Source::Web)]
    source: Source,

    /// picks a random wallpaper
    #[arg(short, long, default_value_t = false)]
    random: bool,
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
            Some(Source::Web) | None if !args.from_config => {
                search(api::Wallhaven, args.query, &connection, true, true).await
            }

            Some(Source::Local) | None => {
                println!("setting local");

                let configured_wall = args
                    .from_config
                    .then(|| {
                        let config = WallpaperConfig::get();
                        println!("from config!");

                        match config.wallpaper.as_ref() {
                            Some(WallpaperPath::Path(path)) => Some(path.to_path_buf()),
                            _ => None,
                        }
                    })
                    .flatten();

                if let Some(path) = configured_wall {
                    println!("got config wall {}", path.display());
                    let wallpaper = Wallpaper {
                        source: LocalSource { path: path.clone() },
                        image: WallpaperImage::from_path(&path)?,
                        tags: Vec::new(),
                    };

                    wallpaper.set_wallpaper().await.map(|_| ())
                } else {
                    search(
                        api::Local::new(&connection),
                        args.query,
                        &connection,
                        true,
                        false,
                    )
                    .await
                }
            }

            _ => unreachable!("from_config can't be used with Source::Web"),
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
    Wallpaper<T::Source>: Fetch<Origin = T::Source> + WallpaperSetter,
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
                WallpaperPath::Url(url) => url,
                WallpaperPath::Path(path) => path.display().to_string(),
            };
            println!("{}\t{location}", wallpaper.image.name);
        }
        return Ok(());
    }

    if let Some(wallpaper) = wallpapers.into_iter().next() {
        println!("Setting {}", wallpaper.image.name);

        if register {
            let mem = wallpaper.fetch().await?.set_wallpaper().await?;
            let local = mem.save().await?;
            db::add_wallpaper(conn, local)?;
        } else {
            wallpaper.set_wallpaper().await?;
        }
    }
    Ok(())
}
