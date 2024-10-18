use super::{TryGet, BOARDS, BOARDS_BY_USER, DB};
use crate::data::BoardDB;
use bincode::serialize;
use redb::MultimapValue;
use eyre::{ContextCompat, OptionExt, Result, WrapErr};

pub fn check_board(board_id: u64) -> Result<()> {
    let db = DB.try_get()?;
    let db =db.db.clone();
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
