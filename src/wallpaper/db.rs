use crate::wallpaper::source::LocalSource;

use super::Wallpaper;
use rusqlite::{Connection, Result};
use std::path::Path;

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

        CREATE INDEX idx_collection_tag ON Collection(tag_id)
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
