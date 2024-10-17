use serde::{Deserialize, Serialize};
use small_uid::SmallUid;

#[derive(Serialize, Deserialize)]
pub struct Board {
    pub user: String,
    pub id: String,
    pub name: String,
    pub messages: Vec<Message>,
}

#[derive(Serialize, Deserialize)]
pub struct Message {
    pub board_id: String,
    pub id: String,
    pub message: String,
    pub reply: Vec<Reply>,
}

#[derive(Serialize, Deserialize)]
pub struct Reply {
    pub message_id: String,
    pub id: String,
    pub reply: String,
}

#[derive(Serialize, Deserialize)]
pub struct BoardDB {
    pub user: SmallUid,
    pub id: SmallUid,
    pub name: String,
}

#[derive(Serialize, Deserialize)]
pub struct MessageDB {
    pub board_id: SmallUid,
    pub id: SmallUid,
    pub message: String,
}

#[derive(Serialize, Deserialize)]
pub struct ReplyDB {
    pub message_id: SmallUid,
    pub id: SmallUid,
    pub reply: String,
}
