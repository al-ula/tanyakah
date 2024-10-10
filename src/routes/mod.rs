pub mod htmx;

use std::collections::HashMap;
use salvo::{handler, Response};
use salvo::prelude::Text;
use serde_json::json;
use crate::render;

#[handler]
pub async fn index(response: &mut Response) {
    let components = HashMap::from([
        ("index".to_string(), render::get_component("main_layout")),
        ("content".to_string(), render::get_component("register")),
    ]);
    let page = render::render_layout(
        "index",
        components,
        &json!({
            "site": "Tanyakah",
            "title": "Tanyakah"
        }),
    )
        .await;
    response.render(Text::Html(page));
}

#[handler]
pub async fn register(response: &mut Response) {
    let components = HashMap::from([
        ("daftar".to_string(), render::get_component("main_layout")),
        ("content".to_string(), render::get_component("register")),
    ]);
    let page = render::render_layout(
        "daftar",
        components,
        &json!({
            "site": "Tanyakah",
            "title": "Daftar ke Tanyakah"
        }),
    )
        .await;
    response.render(Text::Html(page));
}

#[handler]
pub async fn board(response: &mut Response) {
    let components = HashMap::from([
        ("papan".to_string(), render::get_component("main_layout")),
        ("content".to_string(), render::get_component("board")),
        ("message_list".to_string(), render::get_component("message")),
        ("reply".to_string(), render::get_component("message_reply")),
    ]);
    let page = render::render_layout(
        "papan",
        components,
        &json!({
            "site": "Tanyakah",
            "title": "Papan Pesan Rahasia",
            "messages": [
                {"message": "papan", "reply": ["asdf", "sdafsdf"]}
            ]
        }),
    )
        .await;
    response.render(Text::Html(page));
}

#[handler]
pub async fn message(response: &mut Response) {
    let components = HashMap::from([
        ("pesan".to_string(), render::get_component("main_layout")),
        ("content".to_string(), render::get_component("message")),
        ("reply".to_string(), render::get_component("message_reply")),
    ]);
    let page = render::render_layout(
        "pesan",
        components,
        &json!({
            "site": "Tanyakah",
            "title": "Papan Pesan Rahasia",
            "reply": [
                "pesan"
            ]
        }),
    )
        .await;
    response.render(Text::Html(page));
}