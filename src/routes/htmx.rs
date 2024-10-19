use crate::render;
use salvo::prelude::{StatusCode, Text};
use salvo::{handler, Request, Response};
use serde::de::value::Error;
use serde::Deserialize;
use serde_json::json;
use std::collections::HashMap;
use tracing::{debug, warn};

#[derive(Deserialize)]
struct FormData {
    username: String,
}

#[handler]
pub async fn register_post(req: &mut Request, res: &mut Response) {
    let body = req.payload().await.unwrap();
    let body_string = String::from_utf8(body.to_vec()).unwrap();
    if req.headers().get("hx-request").map(|h| h.to_str().unwrap()) != Some("true") {
        warn!("Not htmx request");
        res.status_code(StatusCode::BAD_REQUEST);
        return;
    }
    let form: Result<FormData, Error> = serde_urlencoded::from_str(&body_string);
    match form {
        Ok(form) => {
            let username = form.username;
            debug!("{}", username);
            let components = HashMap::from([
                ("papan".to_string(), render::get_component("board")),
                (
                    "message_list".to_string(),
                    render::get_component("message_list"),
                ),
                ("reply".to_string(), render::get_component("message_reply")),
            ]);
            let page = render::render_layout("papan", components, &json!({})).await;
            res.render(Text::Html(page));
        }
        Err(e) => {
            warn!("{:?}", e);
            res.status_code(StatusCode::BAD_REQUEST);
        }
    }
}

#[handler]
pub async fn send_message() {
    // TODO
}

#[handler]
pub async fn send_reply() {
    // TODO
}
