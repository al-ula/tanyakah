use bincode::{deserialize, serialize};
use redb::MultimapValue;
use crate::data::MessageDB;
use super::{TryGet, DB, MESSAGES, MESSAGES_BY_BOARD};
use eyre::{OptionExt, Result};

pub fn check_message(message_id: u64) -> Result<()> {
    let db = DB.try_get()?.db.clone();
    let read_txn = db.begin_read()?;
    let table = read_txn.open_table(MESSAGES)?;
    table.get(message_id)?;
    Ok(())
}

pub fn insert_message(message: MessageDB) -> Result<()> {
    let board_id: u64 = message.board_id.into();
    let id: u64 = message.id.into();
    let data: &[u8] = &serialize(&message)?;
    let db = DB.try_get()?.db.clone();
    let write_txn = db.begin_write()?;
    {
        let mut table1 = write_txn.open_table(MESSAGES)?;
        table1.insert(id, data)?;
        let mut table2 = write_txn.open_multimap_table(MESSAGES_BY_BOARD)?;
        table2.insert(board_id, id)?;
    }
    write_txn.commit()?;
    Ok(())
}

pub fn get_message(message_id: u64) -> Result<MessageDB> {
    let db = DB.try_get()?.db.clone();
    let read_txn = db.begin_read()?;    
    let table = read_txn.open_table(MESSAGES)?;
    let data = table.get(message_id)?.ok_or_eyre("Message not found")?; 
    let message: MessageDB = deserialize(data.value())?;
    Ok(message)
}

pub fn get_messages_list(board_id: u64) -> Result<Vec<u64>> {
    let db = DB.try_get()?.db.clone();
    let read_txn = db.begin_read()?;    
    let table = read_txn.open_multimap_table(MESSAGES_BY_BOARD)?;
    let values: MultimapValue<u64> = table.get(board_id)?;
    let mut vec = Vec::new();
    for value in values {
        let value = value?.value();
        vec.push(value);
    }
    Ok(vec)
}

pub fn get_messages(board_id: u64) -> Result<Vec<MessageDB>> {
    let message_list = get_messages_list(board_id)?;
    let mut vec = Vec::new();
    for message_id in message_list {
        let message = get_message(message_id)?;
        vec.push(message);
    }
    Ok(vec)
}