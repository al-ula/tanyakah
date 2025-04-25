pub mod htmx;

use crate::auth::SimpleAuth;
use crate::db::boards::{full_board, full_board_preview, get_board};
use crate::db::messages::full_message;
use crate::render;
use salvo::prelude::{StatusCode, Text};
use salvo::{handler, Request, Response};
use serde_json::json;
use small_uid::SmallUid;
use std::collections::HashMap;
use tracing::info;

#[handler]
pub async fn index(req: &mut Request, response: &mut Response){
    if let Some(data) = req.extensions().get::<SimpleAuth>() {
        info!("data: {:#?}", data.clone());
        let board_id: u64 = SmallUid::try_from(data.board.clone()).unwrap().into();
        info!("board_id: {:#?}", board_id);
        let boarddb = get_board(board_id).unwrap();
        let content = render::sections::board_home(boarddb).await;
        let components = HashMap::from([("index".to_string(), render::get_component("layout"))]);
        let component_string = HashMap::from([("content".to_string(), content)]);
        let page = render::render_layout(
            "index",
            components,
            component_string,
            &json!({
                "site": "Tanyakah",
                "title": "Berbagi pesan rahasia di Tanyakah",
                "description": "Dapatkan pesan rahasia dari teman-temanmu kalau kamu punya!."
            }),
        )
        .await;
        response.render(Text::Html(page));
        return;
    }

    let components = HashMap::from([
        ("index".to_string(), render::get_component("layout")),
        ("content".to_string(), render::get_component("register")),
    ]);
    let page = render::render_layout(
        "index",
        components,
        HashMap::new(),
        &json!({
            "site": "Tanyakah",
            "title": "Berbagi pesan rahasia di Tanyakah",
            "description": "Dapatkan pesan rahasia dari teman-temanmu kalau kamu punya!."
        }),
    )
    .await;
    response.render(Text::Html(page));
}

#[handler]
pub async fn board_view(req: &mut Request, res: &mut Response) {
    let param = req.param::<String>("board_id").unwrap();
    let param = format!("{}", param);
    info!("Handling board view: {}", param);
    let board_id = match SmallUid::try_from(param.clone()) {
        Ok(b) => b,
        Err(e) => {
            info!("board_view::board_id: {:#?}", e);
            res.status_code(StatusCode::NOT_FOUND);
            return;
        }
    };
    let auth_board = req
        .extensions()
        .get::<SimpleAuth>()
        .map(|a| SmallUid::try_from(a.board.clone()).unwrap());

    let is_owned: bool = match auth_board {
        Some(a) => a == board_id,
        None => false,
    };
    info!("Start querying board");
    let board_id: u64 = board_id.into();
    let full_board = match full_board_preview(board_id) {
        Ok(b) => b,
        Err(e) => {
            info!("board_view::full_board: {:#?}", e);
            res.status_code(StatusCode::NOT_FOUND);
            return;
        }
    };
    let description: String;
    let content = match is_owned {
        true => {
            description = "Dapatkan pesan rahasia dari teman-temanmu kalau kamu punya!".to_string();
            render::sections::myboard(full_board).await
        }
        false => {
            description = format!("Kirim pesan rahasia untuk {}", full_board.name);
            render::sections::board(full_board).await
        }
    };

    let components = HashMap::from([("index".to_string(), render::get_component("layout"))]);
    let component_string = HashMap::from([("content".to_string(), content)]);
    info!("Rendering");
    let page = render::render_layout(
        "index",
        components,
        component_string,
        &json!({
            "site": "Tanyakah",
            "title": "Berbagi pesan rahasia di Tanyakah",
            "description": description
        }),
    )
    .await;
    res.render(Text::Html(page));
    info!("{} rendered", param);
    return;
}

#[handler]
pub async fn message_view(req: &mut Request, res: &mut Response) {
    let message_id = SmallUid::try_from(req.param::<String>("msg_id").unwrap())
        .unwrap()
        .0;
    let message = full_message(message_id).unwrap();
    let board_id = message.board_id.clone();
    let content = render::sections::message_page(board_id, message).await;
    let components = HashMap::from([("index".to_string(), render::get_component("layout"))]);
    let component_string = HashMap::from([("content".to_string(), content)]);
    let page = render::render_layout(
        "index",
        components,
        component_string,
        &json!({
            "site": "Tanyakah",
            "title": "Berbagi pesan rahasia di Tanyakah",
            "description": "Kirimlah pesan rahasiauntuk temanmu!."
        }),
    )
    .await;
    res.render(Text::Html(page));
}
