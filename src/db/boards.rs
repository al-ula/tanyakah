use super::{TryGet, BOARDS, BOARDS_BY_USER, DB};
use crate::data::{Board, BoardDB};
use crate::db::messages::{
    del_message, full_message, full_messages_preview, get_messages, get_messages_list,
};
use bincode::serialize;
use eyre::{OptionExt, Result, WrapErr};
use log::info;
use redb::MultimapValue;

pub fn check_board(board_id: u64) -> Result<()> {
    let db = DB.try_get()?;
    let db = db.db.clone();
    let read_txn = db.begin_read()?;
    let table = read_txn.open_table(BOARDS)?;
    table.get(board_id)?;
    Ok(())
}

pub fn insert_board(board: BoardDB) -> Result<()> {
    let user: u64 = board.user.into();
    let id: u64 = board.id.into();
    let data: &[u8] = &serialize(&board)?;
    let db = DB.try_get()?.db.clone();
    let write_txn = db.begin_write()?;
    {
        let mut table1 = write_txn.open_table(BOARDS)?;
        table1.insert(id, data)?;
        let mut table2 = write_txn.open_multimap_table(BOARDS_BY_USER)?;
        table2.insert(user, id)?;
    }
    write_txn.commit()?;
    Ok(())
}

pub fn get_board(board_id: u64) -> Result<BoardDB> {
    let db = DB.try_get()?.db.clone();
    let read_txn = db.begin_read()?;
    let table = read_txn.open_table(BOARDS)?;
    let data = table.get(board_id)?.ok_or_eyre("Board not found")?;
    let data = data.value();
    let board = bincode::deserialize(data)?;
    Ok(board)
}

pub fn get_boards_list(user_id: u64) -> Result<Vec<u64>> {
    let db = DB.try_get()?.db.clone();
    let read_txn = db.begin_read()?;
    let table = read_txn.open_multimap_table(BOARDS_BY_USER)?;
    let values: MultimapValue<u64> = table.get(user_id)?;
    let mut vec = Vec::new();
    for value in values {
        let value = value?.value();
        vec.push(value);
    }
    Ok(vec)
}

pub fn get_boards(user_id: u64) -> Result<Vec<BoardDB>> {
    let board_list = get_boards_list(user_id)?;
    let mut vec = Vec::new();
    for board_id in board_list {
        let board = get_board(board_id)?;
        vec.push(board);
    }
    Ok(vec)
}

pub fn del_board(id: u64) -> Result<()> {
    let db = DB.try_get()?.db.clone();
    let user_id: u64 = get_board(id)?.user.into();
    let messages = get_messages_list(id)?;
    for message_id in messages {
        del_message(message_id)?;
    }
    let write_txn = db.begin_write()?;
    {
        let mut table = write_txn.open_table(BOARDS)?;
        table.remove(id)?;

        let mut table = write_txn.open_multimap_table(BOARDS_BY_USER)?;
        table.remove(user_id, id)?;
    }
    write_txn.commit()?;
    Ok(())
}

pub fn full_board(board_id: u64) -> Result<Board> {
    let board = get_board(board_id)?;
    let mut messages = Vec::new();
    let is_messaged = get_messages_list(board_id).is_ok();
    info!("is_messaged: {:?}", is_messaged);
    if is_messaged {
        for message_id in get_messages_list(board_id)? {
            messages.push(full_message(message_id)?);
        }
    }
    Ok(board.to_board(messages))
}

pub fn full_board_preview(board_id: u64) -> Result<Board> {
    let board = get_board(board_id)?;
    let mut messages = Vec::new();
    let is_messaged = get_messages_list(board_id).is_ok();
    info!("is_messaged: {:?}", is_messaged);
    if is_messaged {
        messages = full_messages_preview(board_id)?;
    }
    Ok(board.to_board(messages))
}
