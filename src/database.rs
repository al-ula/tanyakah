use redb::{Database, MultimapTableDefinition, TableDefinition};
use std::path::PathBuf;
use std::sync::{Arc, OnceLock};

pub struct Db {
    pub db: Arc<Database>,
}

impl Db {
    pub fn init(path_buf: PathBuf) -> Result<Db, redb::Error> {
        let db = Database::create(path_buf)?;
        Ok(Self { db: Arc::new(db) })
    }
}

pub static DB: OnceLock<Db> = OnceLock::new();

pub fn initialize_db(path: PathBuf) -> Result<(), redb::Error> {
    let db = Db::init(path)?;
    DB.set(db).ok();
    Ok(())
}

const BOARDS: TableDefinition<u128, &[u8]> = TableDefinition::new("boards");
const MESSAGES: MultimapTableDefinition<u128, &[u8]> = MultimapTableDefinition::new("messages");
const REPLIES: MultimapTableDefinition<u128, String> = MultimapTableDefinition::new("replies");
