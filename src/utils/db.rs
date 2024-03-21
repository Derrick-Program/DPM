use futures_util::StreamExt;
use reqwest::Error;
use rusqlite::{params, Connection, Result};
use std::{collections::HashMap, path::Path};
use tokio::fs::File;
use tokio::io::AsyncWriteExt;

use crate::{MyError, MyResult};

#[derive(Debug)]
pub struct Db {
    conn: Connection,
}
#[derive(Debug)]
pub struct DbPackage {
    name: String,
    version: String,
    url: String,
    description: String,
    filename: String,
    hash: String,
    entry: String,
}
impl DbPackage {
    pub fn new(
        name: &str,
        version: &str,
        url: &str,
        description: &str,
        filename: &str,
        hash: &str,
        entry: &str,
    ) -> Self {
        DbPackage {
            name: name.to_owned(),
            version: version.to_owned(),
            url: url.to_owned(),
            description: description.to_owned(),
            filename: filename.to_owned(),
            hash: hash.to_owned(),
            entry: entry.to_owned(),
        }
    }
}

impl Db {
    pub fn new(file: &Path) -> Result<Self> {
        let saver = file.join("LocalRepo.db");
        let conn = Connection::open(saver)?;
        Ok(Db { conn })
    }

    pub fn create_table(&self) -> Result<()> {
        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS LocalRepo (
                    name            TEXT PRIMARY KEY NOT NULL UNIQUE,
                    version         TEXT NOT NULL,
                    url             TEXT NOT NULL,
                    description     TEXT NOT NULL,
                    filename        TEXT NOT NULL,
                    hash            TEXT NOT NULL,
                    entry           TEXT NOT NULL
                )",
            [],
        )?;
        Ok(())
    }

    pub fn insert(&self, pkg: DbPackage) -> Result<()> {
        self.conn.execute(
            "INSERT INTO LocalRepo (name, version, url,description,filename,hash,entry) VALUES (?1, ?2, ?3,?4,?5,?6,?7)",
            [pkg.name, pkg.version, pkg.url,pkg.description,pkg.filename,pkg.hash,pkg.entry],
        )?;
        Ok(())
    }

    pub fn read_all(&self) -> Result<HashMap<String, DbPackage>> {
        let mut stmt = self.conn.prepare("SELECT * FROM LocalRepo")?;
        let software_iter = stmt.query_map([], |row| {
            Ok((
                row.get::<_, String>(0)?, // version
                row.get::<_, String>(1)?,
                row.get::<_, String>(2)?,
                row.get::<_, String>(3)?,
                row.get::<_, String>(4)?,
                row.get::<_, String>(5)?,
                row.get::<_, String>(6)?,
            ))
        })?;

        let mut software_map = HashMap::<String, DbPackage>::new();
        for result in software_iter {
            let (name, version, url, description, filename, hash, entry) = result?;
            software_map.insert(
                name.clone(),
                DbPackage::new(
                    &name,
                    &version,
                    &url,
                    &description,
                    &filename,
                    &hash,
                    &entry,
                ),
            );
        }
        Ok(software_map)
    }
    pub fn read_one(&self, name: &str) -> Result<HashMap<String, DbPackage>> {
        let mut stmt = self
            .conn
            .prepare("SELECT * FROM LocalRepo WHERE name = ?1")?;
        let software_iter = stmt.query_map([name], |row| {
            Ok((
                row.get::<_, String>(0)?, // version
                row.get::<_, String>(1)?,
                row.get::<_, String>(2)?,
                row.get::<_, String>(3)?,
                row.get::<_, String>(4)?,
                row.get::<_, String>(5)?,
                row.get::<_, String>(6)?,
            ))
        })?;

        let mut software_map = HashMap::<String, DbPackage>::new();
        for result in software_iter {
            let (name, version, url, description, filename, hash, entry) = result?;
            software_map.insert(
                name.clone(),
                DbPackage::new(
                    &name,
                    &version,
                    &url,
                    &description,
                    &filename,
                    &hash,
                    &entry,
                ),
            );
        }
        Ok(software_map)
    }

    pub fn read_one_field(&self, field: &str, name: &str) -> Result<String> {
        let mut stmt = self
            .conn
            .prepare(&format!("SELECT {} FROM LocalRepo WHERE name = ?1", field))?;
        let value = stmt.query_row([name], |row| row.get(0))?;
        Ok(value)
    }

    pub fn update_version(&self, name: &str, new_version: &str) -> Result<()> {
        self.conn.execute(
            "UPDATE LocalRepo SET version = ?1 WHERE name = ?2",
            [new_version, name],
        )?;
        Ok(())
    }

    pub fn delete(&self, name: &str) -> Result<()> {
        self.conn
            .execute("DELETE FROM LocalRepo WHERE name = ?1", [name])?;
        Ok(())
    }
    pub async fn download_file(&self, name: &str) -> MyResult<()> {
        let url = self.read_one_field("url", name).unwrap();
        let req = reqwest::get(url).await?;
        if !req.status().is_success() {
            return Err(Box::new(MyError::new(
                format!("Failed to download file: HTTP {}", req.status()).as_str(),
            )));
        }
        let filename = Path::new("/tmp").join(self.read_one_field("filename", name).unwrap());
        let mut file = File::create(filename).await?;

        let mut stream = req.bytes_stream();

        while let Some(item) = stream.next().await {
            let chunk = item?;
            file.write_all(&chunk).await?;
        }
        Ok(())
    }
}
