use super::{TryGet, DB, MESSAGES, MESSAGES_BY_BOARD};
use crate::data::{Message, MessageDB, Reply};
use crate::db::replies::{del_reply, get_replies, get_reply_list};
use bincode::{deserialize, serialize};
use eyre::{OptionExt, Result};
use log::info;
use redb::MultimapValue;

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

pub fn del_message(id: u64) -> Result<()> {
    let db = DB.try_get()?.db.clone();
    let board_id: u64 = get_message(id)?.board_id.into();
    let replies = get_reply_list(id)?;
    for reply_id in replies {
        del_reply(reply_id)?;
    }
    let write_txn = db.begin_write()?;
    {
        let mut table = write_txn.open_table(MESSAGES)?;
        table.remove(id)?;

        let mut table = write_txn.open_multimap_table(MESSAGES_BY_BOARD)?;
        table.remove(board_id, id)?;
    }
    write_txn.commit()?;
    Ok(())
}

pub fn full_message(message_id: u64) -> Result<Message> {
    let replies = get_replies(message_id);
    let replies = match replies {
        Ok(replies) => replies.into_iter().map(Reply::from).collect(),
        Err(e) => {
            info!("Failed to get replies: {}", e);
            Vec::new()
        }
    };
    Ok(get_message(message_id)?.to_message(replies))
}

pub fn full_messages(board_id: u64) -> Result<Vec<Message>> {
    let mut vec = Vec::new();
    let messages = get_messages(board_id)?;
    for message in messages {
        let replies = match get_replies(message.id.into()) {
            Ok(replies) => replies.into_iter().map(Reply::from).collect(),
            Err(e) => {
                info!("Failed to get replies: {}", e);
                Vec::new()
            }
        };
        let message = message.to_message(replies);
        vec.push(message);
    }
    Ok(vec)
}

pub fn full_messages_preview(board_id: u64) -> Result<Vec<Message>> {
    let mut vec = Vec::new();
    let messages = get_messages(board_id)?;
    for message in messages {
        let replies = match get_replies(message.id.into()) {
            Ok(replies) => replies.into_iter().map(Reply::from).take(2).collect(),
            Err(e) => {
                info!("Failed to get replies: {}", e);
                Vec::new()
            }
        };
        let message = message.to_message(replies);
        vec.push(message);
    }
    Ok(vec)
}

pub fn full_message_preview(message_id: u64) -> Result<Message> {
    let replies = get_replies(message_id);
    let replies = match replies {
        Ok(replies) => replies.into_iter().map(Reply::from).take(2).collect(),
        Err(e) => {
            info!("Failed to get replies: {}", e);
            Vec::new()
        }
    };
    Ok(get_message(message_id)?.to_message(replies))
}
