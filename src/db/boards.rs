use super::{BOARDS, BOARDS_BY_USER, DB};
use crate::data::BoardDB;
use bincode::serialize;
use redb::MultimapValue;

pub fn check_board_id(board_id: u64) -> Result<(), redb::Error> {
    let db = DB.get().unwrap().db.clone();
    let read_txn = db.begin_read()?;
    let table = read_txn.open_table(BOARDS)?;
    table.get(board_id)?;
    Ok(())
}

pub fn insert_board(board: BoardDB) -> Result<(), redb::Error> {
    let user: u64 = board.user.into();
    let id: u64 = board.id.into();
    check_board_id(id)?;
    let data: &[u8] = &serialize(&board).unwrap();
    let db = DB.get().unwrap().db.clone();
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

pub fn get_board(board_id: u64) -> Result<BoardDB, redb::Error> {
    let db = DB.get().unwrap().db.clone();
    let read_txn = db.begin_read()?;
    let table = read_txn.open_table(BOARDS)?;
    let data = table.get(board_id)?.unwrap();
    let data = data.value();
    let board = bincode::deserialize(&data).unwrap();
    Ok(board)
}

pub fn get_board_id_by_user(user_id: u64) -> Result<Vec<u64>, redb::Error> {
    let db = DB.get().unwrap().db.clone();
    let read_txn = db.begin_read()?;
    let table = read_txn.open_multimap_table(BOARDS_BY_USER)?;
    let values: MultimapValue<u64> = table.get(user_id)?;
    let mut vec = Vec::new();
    for value in values {
        let value = value?.value();
        vec.push(value.into());
    }
    Ok(vec)
}

pub fn get_boards_by_user(user_id: u64) -> Result<Vec<BoardDB>, redb::Error> {
    let board_ids = get_board_id_by_user(user_id)?;
    let mut vec = Vec::new();
    for board_id in board_ids {
        let board = get_board(board_id)?;
        vec.push(board);
    }
    Ok(vec)
}
