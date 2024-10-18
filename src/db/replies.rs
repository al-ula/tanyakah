use crate::data::ReplyDB;
use crate::db::{TryGet, DB, REPLIES, REPLIES_BY_MESSAGE};
use bincode::{deserialize, serialize};
use eyre::Result;
use redb::MultimapValue;

pub fn check_reply(reply_id: u64) -> Result<()> {
    let db = DB.try_get()?.db.clone();
    let read_txn = db.begin_read()?;
    let table = read_txn.open_table(REPLIES)?;
    table.get(reply_id)?;
    Ok(())
}

pub fn insert_reply(reply: ReplyDB) -> Result<()> {
    let id: u64 = reply.id.into();
    let data: &[u8] = &serialize(&reply)?;
    let db = DB.try_get()?.db.clone();
    let write_txn = db.begin_write()?;
    {
        let mut table = write_txn.open_table(REPLIES)?;
        table.insert(id, data)?;
    }
    write_txn.commit()?;
    Ok(())
}

pub fn get_reply(id: u64) -> Result<ReplyDB> {
    let db = DB.try_get()?.db.clone();
    let read_txn = db.begin_read()?;
    let table = read_txn.open_table(REPLIES)?;
    let value = table.get(id)?.ok_or(eyre::eyre!("No such reply"))?;
    let reply = deserialize(value.value())?;
    Ok(reply)
}

pub fn get_reply_list(message_id: u64) -> Result<Vec<u64>> {
    let db = DB.try_get()?.db.clone();
    let read_txn = db.begin_read()?;
    let table = read_txn.open_multimap_table(REPLIES_BY_MESSAGE)?;
    let values: MultimapValue<u64> = table.get(message_id)?;
    let mut vec = Vec::new();
    for value in values {
        let value = value?.value();
        vec.push(value);
    }
    Ok(vec)
}

pub fn get_replies(message_id: u64) -> Result<Vec<ReplyDB>> {
    let reply_list = get_reply_list(message_id)?;
    let mut vec = Vec::new();
    for reply_id in reply_list {
        let reply = get_reply(reply_id)?;
        vec.push(reply);
    }
    Ok(vec)
}
