mod boards;
mod messages;
mod replies;

use redb::{Database, MultimapTableDefinition, TableDefinition};
use std::path::PathBuf;
use std::sync::{Arc, OnceLock};
use eyre::{Result, WrapErr};

#[derive(Clone)]
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

pub trait TryGet<T> {
    fn try_get(&self) -> Result<&T>
    where
        Self: Sized,
        T: std::clone::Clone;
}

impl TryGet<Db> for OnceLock<Db> {
    fn try_get(&self) -> Result<&Db> {
        let db = self.get()
            .ok_or_else(|| eyre::eyre!("DB is not initialized"))?;
        Ok(db)
    }
}

pub const BOARDS: TableDefinition<u64, &[u8]> = TableDefinition::new("boards");
pub const BOARDS_BY_USER: MultimapTableDefinition<u64, u64> =
    MultimapTableDefinition::new("boards_by_user");
pub const MESSAGES: TableDefinition<u64, &[u8]> = TableDefinition::new("messages");
pub const MESSAGES_BY_BOARD: MultimapTableDefinition<u64, u64> =
    MultimapTableDefinition::new("messages_by_board");
pub const REPLIES: TableDefinition<u64, &[u8]> = TableDefinition::new("replies");
pub const REPLIES_BY_MESSAGE: MultimapTableDefinition<u64, u64> =
    MultimapTableDefinition::new("replies_by_message");


