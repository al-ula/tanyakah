use crate::data::{MessageDB, ReplyDB};
use crate::db::messages::{full_message, full_message_preview};
use crate::db::replies::insert_reply;
use crate::render;
use salvo::http::cookie::Cookie;
use salvo::prelude::{StatusCode, Text};
use salvo::{handler, Request, Response};
use serde::de::value::Error;
use serde::Deserialize;
use small_uid::SmallUid;
use tracing::{info, warn};
use crate::db::boards::check_board;

#[derive(Debug, Deserialize)]
struct RegisterForm {
    username: String,
}

#[handler]
pub async fn register(req: &mut Request, res: &mut Response) {
    let body = req.payload().await.unwrap();
    let body_string = String::from_utf8(body.to_vec()).unwrap();
    let form: Result<RegisterForm, Error> = serde_urlencoded::from_str(&body_string);
    match form {
        Ok(form) => {
            let username = match form.username.trim() {
                "" => {
                    res.status_code(StatusCode::BAD_REQUEST);
                    return;
                }
                u => u.to_string(),
            };
            let board = crate::db::create_board(username.clone()).unwrap();
            if check_board(board.id.0).is_ok() {
                warn!("register: board already exists");
                res.status_code(StatusCode::BAD_REQUEST);
                return;
            }
            crate::db::register_board(board.clone()).unwrap();
            info!("{}", username);
            info!("{:?}", board);
            let page = render::sections::board_home(board.clone()).await;
            res.render(Text::Html(page));
            let auth = crate::auth::SimpleAuth::new(
                board.user.into(),
                board.id.into(),
                chrono::Utc::now().timestamp() as usize,
            );
            let cookie = Cookie::new("auth_token_token", auth.token().unwrap());
            res.add_cookie(cookie);
        }
        Err(e) => {
            info!("register: {:?}", e);
            res.status_code(StatusCode::BAD_REQUEST);
        }
    }
}

#[derive(Debug, Deserialize)]
struct SendMessageForm {
    board_id: String,
    message: String,
}

#[handler]
pub async fn send_message(req: &mut Request, res: &mut Response) {
    let body = req.payload().await.unwrap();
    let body_string = String::from_utf8(body.to_vec()).unwrap();
    let form: Result<SendMessageForm, Error> = serde_urlencoded::from_str(&body_string);
    match form {
        Ok(form) => {
            let board_id: SmallUid = match SmallUid::try_from(form.board_id) {
                Ok(id) => id,
                Err(_) => {
                    res.status_code(StatusCode::BAD_REQUEST);
                    return;
                }
            };
            let message = match form.message.trim() {
                "" => {
                    res.status_code(StatusCode::BAD_REQUEST);
                    return;
                }
                message_trim => message_trim.to_string(),
            };

            let messagedb = match MessageDB::new(board_id.into(), message) {
                Ok(messagedb) => messagedb,
                Err(e) => {
                    warn!("send_message::message_query: {:?}", e);
                    res.status_code(StatusCode::INTERNAL_SERVER_ERROR);
                    return;
                }
            };

            match crate::db::messages::insert_message(messagedb) {
                Ok(_) => {}
                Err(e) => {
                    warn!("send_message::message_insert: {:?}", e);
                    res.status_code(StatusCode::INTERNAL_SERVER_ERROR);
                    return;
                }
            };

            let messages = match crate::db::messages::full_messages_preview(board_id.into()) {
                Ok(messages) => messages,
                Err(e) => {
                    warn!("send_message::messages_query: {:?}", e);
                    res.status_code(StatusCode::BAD_REQUEST);
                    return;
                }
            };

            let page = render::sections::messages(board_id.to_string(), messages).await;
            res.render(Text::Html(page));
        }
        Err(e) => {
            warn!("send_message: {:?}", e);
            res.status_code(StatusCode::BAD_REQUEST);
        }
    }
}

#[handler]
pub async fn get_messages(req: &mut Request, res: &mut Response) {
    let headers = req.headers();
    let hx_current_url = match headers.get("hx-current-url") {
        Some(hx_current_url) => hx_current_url,
        None => {
            warn!("get_messages: No hx-current-url header found");
            res.status_code(StatusCode::BAD_REQUEST);
            return;
        }
    }
    .to_str()
    .unwrap();
    info!("{:?}", hx_current_url);
    let current_url = url::Url::parse(hx_current_url).unwrap();
    let board_id = current_url.path_segments().unwrap().last().unwrap();
    let board_id = SmallUid::try_from(board_id.to_string()).unwrap();

    let messages = match crate::db::messages::full_messages_preview(board_id.into()) {
        Ok(messages) => messages,
        Err(e) => {
            warn!("get_messages::messages_query: {:?}", e);
            res.status_code(StatusCode::BAD_REQUEST);
            return;
        }
    };
    let page = render::sections::messages(board_id.to_string(), messages).await;
    res.render(Text::Html(page));
}

#[derive(Debug, Deserialize)]
struct SendReplyForm {
    message_id: String,
    reply: String,
    message_list: bool,
}

#[handler]
pub async fn send_reply(req: &mut Request, res: &mut Response) {
    let body = req.payload().await.unwrap();
    let body_string = String::from_utf8(body.to_vec()).unwrap();
    let form: Result<SendReplyForm, Error> = serde_urlencoded::from_str(&body_string);
    match form {
        Ok(form) => {
            let message_id: SmallUid = match SmallUid::try_from(form.message_id) {
                Ok(id) => id,
                Err(e) => {
                    warn!("Failed to convert message_id: {:?}", e);
                    res.status_code(StatusCode::BAD_REQUEST);
                    return;
                }
            };
            let reply = match form.reply.trim() {
                "" => {
                    res.status_code(StatusCode::BAD_REQUEST);
                    return;
                }
                reply_trim => reply_trim.to_string(),
            };
            let replydb = match ReplyDB::new(message_id, reply) {
                Ok(replydb) => {
                    replydb
                }
                Err(e) => {
                    info!("send_reply::reply_query: {:?}", e);
                    res.status_code(StatusCode::BAD_REQUEST);
                    return;
                }
            };

            let reply = insert_reply(replydb);

            match reply {
                Ok(_) => {}
                Err(e) => {
                    res.status_code(StatusCode::INTERNAL_SERVER_ERROR);
                    warn!("send_reply::reply_insert: {:?}", e);
                    return;
                }
            };

            let replies = crate::db::replies::get_replies(message_id.into());
            info!("{:?}", replies);

            let is_list = form.message_list;
            let message = match is_list {
                true => match full_message_preview(message_id.into()) {
                    Ok(msg) => {
                        msg
                    }
                    Err(e) => {
                        info!("send_reply::message_query: {:?}", e);
                        res.status_code(StatusCode::BAD_REQUEST);
                        return;
                    }
                },
                false => match full_message(message_id.into()) {
                    Ok(msg) => {
                        msg
                    }
                    Err(e) => {
                        info!("send_reply::message_query: {:?}", e);
                        res.status_code(StatusCode::BAD_REQUEST);
                        return;
                    }
                },
            };
            let render = render::sections::message(message, is_list).await;
            res.render(Text::Html(render));
        }
        Err(e) => {
            info!("send_message: {:?}", e);
            res.status_code(StatusCode::BAD_REQUEST);
        }
    }
}
