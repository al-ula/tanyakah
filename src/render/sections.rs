use crate::data::{Board, BoardDB, Message};
use crate::db::boards::get_board;
use crate::render;
use serde_json::json;
use std::collections::HashMap;

pub async fn register() -> String {
    let components = HashMap::from([("register".to_string(), render::get_component("register"))]);
    render::render_layout("register", components, HashMap::new(), &json!({})).await
}

pub async fn board_home(board: BoardDB) -> String {
    let board = board.to_board(vec![]);
    let components = HashMap::from([
        ("board".to_string(), render::get_component("share")),
        ("message".to_string(), render::get_component("message")),
        ("reply".to_string(), render::get_component("reply")),
    ]);
    render::render_layout(
        "board",
        components,
        HashMap::new(),
        &json!({
            "owned": true,
            "board_id": board.id,
            "messages": board.messages,
            "username": board.name.clone(),
            "profile": true
        }),
    )
    .await
}

pub async fn myboard(board: Board) -> String {
    let components = HashMap::from([
        ("my_board".to_string(), render::get_component("board")),
        (
            "message_list".to_string(),
            render::get_component("message_list"),
        ),
        ("share".to_string(), render::get_component("share")),
        ("message".to_string(), render::get_component("message")),
        ("reply".to_string(), render::get_component("reply")),
    ]);
    render::render_layout(
        "my_board",
        components,
        HashMap::new(),
        &json!({
            "owned": true,
            "board_id": board.id,
            "messages": board.messages,
            "username": board.name,
        }),
    )
    .await
}

pub async fn board(board: Board) -> String {
    let components = HashMap::from([
        ("board".to_string(), render::get_component("board")),
        (
            "message_list".to_string(),
            render::get_component("message_list"),
        ),
        (
            "send_message".to_string(),
            render::get_component("send_message"),
        ),
        ("message".to_string(), render::get_component("message")),
        ("reply".to_string(), render::get_component("reply")),
    ]);
    render::render_layout(
        "board",
        components,
        HashMap::new(),
        &json!({
            "owned": false,
            "board_id": board.id,
            "messages": board.messages,
            "user_name": board.name
        }),
    )
    .await
}

pub async fn message(message: Message, is_list: bool) -> String {
    let components = HashMap::from([
        ("msg".to_string(), render::get_component("message")),
        ("reply".to_string(), render::get_component("reply")),
    ]);
    render::render_layout(
        "msg",
        components,
        HashMap::new(),
        &json!({
            "message_list": is_list,
            "message_id": message.id,
            "message": message.message,
            "reply": message.reply
        }),
    )
    .await
}

pub async fn message_page(board_id: String, message: Message) -> String {
    let board_uid = small_uid::SmallUid::try_from(board_id.clone()).unwrap();
    let user_name = get_board(board_uid.0).unwrap().name;
    let components = HashMap::from([
        (
            "msg_page".to_string(),
            render::get_component("message_page"),
        ),
        ("message".to_string(), render::get_component("message")),
        ("reply".to_string(), render::get_component("reply")),
    ]);
    render::render_layout(
        "msg_page",
        components,
        HashMap::new(),
        &json!({
            "user_name": user_name,
            "board_id": board_id,
            "message_id": message.id,
            "message": message.message,
            "reply": message.reply
        }),
    )
    .await
}

pub async fn messages(board_id: String, messages: Vec<Message>) -> String {
    let components = HashMap::from([
        (
            "messages".to_string(),
            render::get_component("message_container"),
        ),
        ("share".to_string(), render::get_component("share")),
        ("message".to_string(), render::get_component("message")),
        ("reply".to_string(), render::get_component("reply")),
    ]);

    render::render_layout(
        "messages",
        components,
        HashMap::new(),
        &json!({
            "board_id": board_id,
            "messages": messages
        }),
    )
    .await
}
