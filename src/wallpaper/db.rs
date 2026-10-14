use crate::wallpaper::{WallpaperDetails, WallpaperImage, source::LocalSource};

use super::Wallpaper;
use image::ImageFormat;
use rusqlite::{Connection, Result};
use std::path::{Path, PathBuf};

pub fn init(path: &Path) -> Result<Connection> {
    println!("setting path={:?}", path);
    let con = Connection::open(path)?;
    con.execute("PRAGMA foreign_keys = ON", ())?;

    create_tables(&con)?;

    Ok(con)
}

fn create_tables(connection: &Connection) -> Result<()> {
    connection.execute_batch(
        "
        CREATE TABLE IF NOT EXISTS Wallpaper (
            id INTEGER PRIMARY KEY,
            path TEXT NOT NULL UNIQUE,
            metadata TEXT
        );

        CREATE TABLE IF NOT EXISTS Tag (
            id INTEGER PRIMARY KEY,
            name TEXT NOT NULL UNIQUE
        );

        CREATE TABLE IF NOT EXISTS Collection (
            wallpaper_id INTEGER NOT NULL,
            tag_id INTEGER NOT NULL,

            PRIMARY KEY (wallpaper_id, tag_id),

            FOREIGN KEY (wallpaper_id)
                REFERENCES Wallpaper(id)
                ON DELETE CASCADE,

            FOREIGN KEY (tag_id)
                REFERENCES Tag(id)
                ON DELETE CASCADE
        );

        CREATE INDEX IF NOT EXISTS idx_collection_tag ON Collection(tag_id)
        ",
    )
}

pub fn add_wallpaper(connection: &Connection, wallpaper: Wallpaper<LocalSource>) -> Result<()> {
    let json = wallpaper
        .image
        .metadata
        .as_ref()
        .map(|meta| serde_json::to_string(meta).unwrap_or_default());

    connection.execute(
        "INSERT INTO Wallpaper (path, metadata)
        VALUES (?1, ?2)",
        (
            wallpaper.source.path.to_string_lossy(),
            json.unwrap_or("{}".into()),
        ),
    )?;

    let wallpaper_id = connection.last_insert_rowid();

    for tag in &wallpaper.tags {
        connection.execute(
            "INSERT INTO TAG (name)
            VALUES (?1)
            ON CONFLICT(name) DO NOTHING",
            [tag],
        )?;

        let tag_id: i64 =
            connection.query_row("SELECT id FROM Tag WHERE name = ?1", [tag], |row| {
                row.get(0)
            })?;

        connection.execute(
            "INSERT INTO Collection (wallpaper_id, tag_id)
            VALUES (?1, ?2)
            ",
            (wallpaper_id, tag_id),
        )?;
    }

    Ok(())
}

pub fn search_wallpaper(
    connection: &Connection,
    needle: &str,
    limit: usize,
) -> Result<Vec<Wallpaper<LocalSource>>> {
    let mut statement = connection.prepare(
        "SELECT DISTINCT w.id, w.path, w.metadata
        FROM Wallpaper w
        LEFT JOIN Collection c ON c.wallpaper_id = w.id
        LEFT JOIN Tag t on t.id = c.tag_id
        WHERE ?1 = ''
            OR t.name LIKE '%' || ?1 || '%'
            OR w.path LIKE '%' || ?1 || '%'
        ORDER BY RANDOM()
        LIMIT ?2
        ",
    )?;

    let rows: Vec<(i64, String, Option<String>)> = statement
        .query_map((needle, limit as i64), |row| {
            Ok((row.get(0)?, row.get(1)?, row.get(2)?))
        })?
        .collect::<Result<_>>()?;

    let mut tag_statement = connection.prepare(
        "SELECT t.name from Tag t
        JOIN Collection c ON c.tag_id = t.id
        WHERE c.wallpaper_id = ?1
        ",
    )?;

    let mut wallpapers = Vec::with_capacity(rows.len());
    for (id, path, metadata_json) in rows {
        let tags: Vec<String> = tag_statement
            .query_map([id], |row| row.get(0))?
            .collect::<Result<_>>()?;

        let metadata: Option<WallpaperDetails> =
            metadata_json.and_then(|json| serde_json::from_str(&json).ok());

        let path = PathBuf::from(path);
        let ratio = metadata
            .as_ref()
            .and_then(|m| m.ratio.parse::<f64>().ok())
            .unwrap_or(0.);

        wallpapers.push(Wallpaper {
            image: WallpaperImage {
                ratio,
                format: ImageFormat::from_path(&path).ok(),
                name: path
                    .file_stem()
                    .map(|s| s.to_string_lossy().into_owned())
                    .unwrap_or_default(),
                metadata,
            },
            tags,
            source: LocalSource { path },
        });
    }

    Ok(wallpapers)
}
