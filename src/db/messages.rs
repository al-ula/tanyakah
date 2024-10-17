use bincode::{deserialize, serialize};
use redb::MultimapValue;
use crate::data::MessageDB;
use super::{DB, MESSAGES, MESSAGES_BY_BOARD};

pub fn check_message(message_id: u64) -> Result<(), redb::Error> {
    let db = DB.get().unwrap().db.clone();
    let read_txn = db.begin_read()?;
    let table = read_txn.open_table(MESSAGES)?;
    table.get(message_id)?;
    Ok(())
}

pub fn insert_message(message: MessageDB) -> Result<(), redb::Error> {
    let board_id: u64 = message.board_id.into();
    let id: u64 = message.id.into();
    check_message(id)?;
    let data: &[u8] = &serialize(&message).unwrap();
    let db = DB.get().unwrap().db.clone();
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

pub fn get_message(message_id: u64) -> Result<MessageDB, redb::Error> {
    let db = DB.get().unwrap().db.clone();
    let read_txn = db.begin_read()?;    
    let table = read_txn.open_table(MESSAGES)?;
    let data = table.get(message_id)?;
    let message: MessageDB = deserialize(&data.unwrap().value()).unwrap();
    Ok(message)
}

pub fn get_messages_list(board_id: u64) -> Result<Vec<u64>, redb::Error> {
    let db = DB.get().unwrap().db.clone();
    let read_txn = db.begin_read()?;    
    let table = read_txn.open_multimap_table(MESSAGES_BY_BOARD)?;
    let values: MultimapValue<u64> = table.get(board_id)?;
    let mut vec = Vec::new();
    for value in values {
        let value = value?.value();
        vec.push(value.into());
    }
    Ok(vec)
}

pub fn get_messages(board_id: u64) -> Result<Vec<MessageDB>, redb::Error> {
    let message_list = get_messages_list(board_id)?;
    let mut vec = Vec::new();
    for message_id in message_list {
        let message = get_message(message_id)?;
        vec.push(message);
    }
    Ok(vec)
}