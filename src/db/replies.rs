use bincode::{deserialize, serialize};
use redb::MultimapValue;
use crate::data::ReplyDB;
use crate::db::{DB, REPLIES, REPLIES_BY_MESSAGE};

pub fn check_reply(reply_id: u64) -> Result<(), redb::Error> {
    let db = DB.get().unwrap().db.clone();
    let read_txn = db.begin_read()?;
    let table = read_txn.open_table(REPLIES)?;
    table.get(reply_id)?;
    Ok(())
}

pub fn insert_reply(reply: ReplyDB) -> Result<(), redb::Error> {
    let id: u64 = reply.id.into();
    check_reply(id)?;
    let data: &[u8] = &serialize(&reply).unwrap();
    let db = DB.get().unwrap().db.clone();
    let write_txn = db.begin_write()?;
    {
        let mut table = write_txn.open_table(REPLIES)?;
        table.insert(id, data)?;
    }
    write_txn.commit()?;
    Ok(())
}

pub fn get_reply(id: u64) -> Result<ReplyDB, redb::Error> {
    let db = DB.get().unwrap().db.clone();
    let read_txn = db.begin_read()?;
    let table = read_txn.open_table(REPLIES)?;
    let value = table.get(id)?.unwrap();
    let reply = deserialize(value.value()).unwrap();
    Ok(reply)
}

pub fn get_reply_list(message_id: u64) -> Result<Vec<u64>, redb::Error> {
    let db = DB.get().unwrap().db.clone();
    let read_txn = db.begin_read()?;
    let table = read_txn.open_multimap_table(REPLIES_BY_MESSAGE)?;
    let values: MultimapValue<u64> = table.get(message_id)?;
    let mut vec = Vec::new();
    for value in values {
        let value = value?.value();
        vec.push(value.into());
    }
    Ok(vec)
}

pub fn get_replies(message_id: u64) -> Result<Vec<ReplyDB>, redb::Error> {
    let reply_list = get_reply_list(message_id)?;
    let mut vec = Vec::new();
    for reply_id in reply_list {
        let reply = get_reply(reply_id)?;
        vec.push(reply);
    }
    Ok(vec)
}