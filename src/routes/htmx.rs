use std::collections::HashMap;
use salvo::{handler, Request, Response};
use salvo::prelude::Text;
use serde::Deserialize;
use serde_json::json;
use crate::render;

#[derive(Deserialize)]
struct FormData {
    username: String,
}

#[handler]
pub async fn register_post(req: &mut Request, res: &mut Response) {
    let body = req.payload().await.unwrap();
    let body_string = String::from_utf8(body.to_vec()).unwrap();
    let form: FormData = serde_urlencoded::from_str(&body_string).unwrap();
    let username = form.username;
    println!("{}", username);
    let components = HashMap::from([
        ("papan".to_string(), render::get_component("board")),
        ("message_list".to_string(), render::get_component("message")),
        ("reply".to_string(), render::get_component("message_reply")),
    ]);
    let page = render::render_layout(
        "papan",
        components,
        &json!({}),
    )
        .await;
    res.render(Text::Html(page));
}