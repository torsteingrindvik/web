use anyhow::{anyhow, Result};
use rusqlite::{params, Connection};

#[derive(Debug, Clone)]
pub struct Document {
    pub slug: String,
    pub html: String,
}

pub struct DbConn(Connection);

pub fn init_and_open_db() -> Result<DbConn> {
    let path = concat!(env!("CARGO_MANIFEST_DIR"), "/db.db3");

    let conn = Connection::open(&path)?;
    println!("Connection open");

    conn.execute(
        "CREATE TABLE IF NOT EXISTS documents (
                  id                INTEGER PRIMARY KEY,
                  slug              TEXT NOT NULL,
                  data              BLOB
                  )",
        params![],
    )?;

    Ok(DbConn(conn))
}

impl DbConn {
    pub fn insert(&self, document: &Document) -> Result<()> {
        self.0.execute(
            "INSERT INTO documents (slug, data) VALUES (?1, ?2)",
            params![document.slug, document.html],
        )?;

        Ok(())
    }

    pub fn get(&self, slug: String) -> Result<Document> {
        let mut statement = self
            .0
            .prepare("SELECT data FROM documents WHERE slug = ?")?;

        let documents: std::result::Result<Vec<Document>, rusqlite::Error> = statement
            .query_map(params![slug], |row| {
                Ok(Document {
                    slug: slug.clone(),
                    html: row.get(0)?,
                })
            })?
            .collect();

        let documents = documents?;

        if let Some(doc) = documents.get(0) {
            Ok(doc.clone())
        } else {
            Err(anyhow!(
                "Expected 1 document, found {} for slug {}",
                documents.len(),
                &slug
            ))
        }
    }

    pub fn get_all_recent(&self, how_many: usize) -> Result<Vec<String>> {
        let mut statement = self.0.prepare(&format!(
            "SELECT slug FROM documents ORDER BY id DESC LIMIT {}",
            how_many
        ))?;

        let documents: std::result::Result<Vec<String>, rusqlite::Error> = statement
            .query_map(params![], |row| Ok(row.get(0)?))?
            .collect();

        Ok(documents?)
    }
}
