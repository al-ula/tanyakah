pub mod htmx;

use crate::render;
use salvo::prelude::Text;
use salvo::{handler, Response};
use serde_json::json;
use std::collections::HashMap;

#[handler]
pub async fn index(response: &mut Response) {
    let components = HashMap::from([
        ("index".to_string(), render::get_component("layout")),
        ("content".to_string(), render::get_component("register")),
    ]);
    let page = render::render_layout(
        "index",
        components,
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
pub async fn profile(response: &mut Response) {
    let components = HashMap::from([
        ("profile".to_string(), render::get_component("layout")),
        ("content".to_string(), render::get_component("share")),
    ]);
    let page = render::render_layout(
        "profile",
        components,
        &json!({
            "site": "Tanyakah",
            "title": "Berbagi pesan rahasia di Tanyakah",
            "description": "Dapatkan pesan rahasia dari teman-temanmu kalau kamu punya!.",
            "profile": true
        }),
    )
    .await;
    response.render(Text::Html(page));
}

#[handler]
pub async fn my_board(response: &mut Response) {
    let components = HashMap::from([
        ("my_board".to_string(), render::get_component("layout")),
        ("content".to_string(), render::get_component("message_list")),
        ("share".to_string(), render::get_component("share")),
        ("message".to_string(), render::get_component("message")),
        ("reply".to_string(), render::get_component("reply")),
    ]);
    let page = render::render_layout(
        "my_board",
        components,
        &json!({
            "site": "Tanyakah",
            "title": "Berbagi pesan rahasia di Tanyakah",
            "description": "Dapatkan pesan rahasia dari teman-temanmu kalau kamu punya!.",
            "owned": true,
            "board_id": "vcweri",
            "messages": [
                {
                    "message": "Hello World",
                    "id": "yuvb",
                    "reply": [
                        {
                        "content": "Hello too",
                        "id": "easf"
                        },
                        {
                        "content": "Hey!",
                        "id": "uiojn"
                        }
                    ]
                },
                {
                    "message": "I am a message",
                    "id": "bter",
                    "reply": [
                        {
                        "content": "This is a reply",
                        "id": "sze"
                        },
                        {
                        "content": "I am a reply too",
                        "id": "salk"
                        }
                    ]
                }
            ]
        }),
    )
    .await;
    response.render(Text::Html(page));
}

#[handler]
pub async fn board(response: &mut Response) {
    let components = HashMap::from([
        ("board".to_string(), render::get_component("layout")),
        ("content".to_string(), render::get_component("board")),
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
    let page = render::render_layout(
        "board",
        components,
        &json!({
            "site": "Tanyakah",
            "title": "Berbagi pesan rahasia di Tanyakah",
            "description": "Dapatkan pesan rahasia dari teman-temanmu kalau kamu punya!.",
            "owned": false,
            "board_id": "vcweri",
            "messages": [
                {
                    "message": "Hello World",
                    "id": "yuvb",
                    "reply": [
                        {
                        "content": "Hello too",
                        "id": "easf"
                        },
                        {
                        "content": "Hey!",
                        "id": "uiojn"
                        }
                    ]
                },
                {
                    "message": "I am a message",
                    "id": "bter",
                    "reply": [
                        {
                        "content": "This is a reply",
                        "id": "sze"
                        },
                        {
                        "content": "I am a reply too",
                        "id": "salk"
                        }
                    ]
                }
            ]
        }),
    )
    .await;
    response.render(Text::Html(page));
}

#[handler]
pub async fn msg_page(response: &mut Response) {
    let components = HashMap::from([
        ("msg_page".to_string(), render::get_component("layout")),
        ("content".to_string(), render::get_component("message_page")),
        ("message".to_string(), render::get_component("message")),
        ("reply".to_string(), render::get_component("reply")),
    ]);
    let page = render::render_layout(
        "msg_page",
        components,
        &json!({
            "site": "Tanyakah",
            "title": "Berbagi pesan rahasia di Tanyakah",
            "description": "Dapatkan pesan rahasia dari teman-temanmu kalau kamu punya!.",
            "board_id": "vcweri",
            "message": "Hello World",
            "id": "yuvb",
            "reply": [
                {
                    "content": "Hello too",
                    "id": "easf"
                },
                {
                    "content": "Hey!",
                    "id": "uiojn"
                }
            ]
        }),
    )
    .await;
    response.render(Text::Html(page));
}
