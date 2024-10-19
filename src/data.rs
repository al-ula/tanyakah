use eyre::{Error, Result};
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

impl Reply {
    pub fn compose(message_id: String, id: String, reply: String) -> Self {
        Reply {
            message_id,
            id,
            reply,
        }
    }
}

impl TryFrom<Reply> for ReplyDB {
    type Error = Error;

    fn try_from(value: Reply) -> Result<Self, Self::Error> {
        let message_id: SmallUid = value.message_id.try_into()?;
        let id: SmallUid = value.id.try_into()?;
        let replydb = ReplyDB {
            message_id,
            id,
            reply: value.reply,
        };
        Ok(replydb)
    }
}

impl ReplyDB {
    pub fn compose(message_id: SmallUid, id: SmallUid, reply: String) -> Self {
        ReplyDB {
            message_id,
            id,
            reply,
        }
    }
    pub fn new(message_id: SmallUid, reply: String) -> Result<Self> {
        let id = SmallUid::new()?;
        Ok(ReplyDB {
            message_id,
            id,
            reply,
        })
    }
}

impl From<ReplyDB> for Reply {
    fn from(value: ReplyDB) -> Self {
        let message_id: String = value.message_id.into();
        let id: String = value.id.into();
        Reply {
            message_id,
            id,
            reply: value.reply,
        }
    }
}

impl Message {
    pub fn new(board_id: String, message: String, reply: Vec<Reply>) -> Result<Self> {
        Ok(Self {
            board_id,
            id: String::from(SmallUid::new()?),
            message,
            reply,
        })
    }

    pub fn compose(board_id: String, id: String, message: String, reply: Vec<Reply>) -> Self {
        Self {
            board_id,
            id,
            message,
            reply,
        }
    }
}

impl TryFrom<Message> for MessageDB {
    type Error = Error;

    fn try_from(value: Message) -> Result<Self, Self::Error> {
        let board_id: SmallUid = value.board_id.try_into()?;
        let id: SmallUid = value.id.try_into()?;
        Ok(MessageDB {
            board_id,
            id,
            message: value.message,
        })
    }
}

impl MessageDB {
    pub fn to_message(&self, replies: Vec<ReplyDB>) -> Message {
        let replies = replies.into_iter().map(Reply::from).collect();
        Message {
            board_id: self.board_id.into(),
            id: self.id.into(),
            message: self.message.clone(),
            reply: replies,
        }
    }

    pub fn new(board_id: SmallUid, message: String) -> Result<Self> {
        let id = SmallUid::new()?;
        Ok(MessageDB {
            board_id,
            id,
            message,
        })
    }

    pub fn compose(board_id: SmallUid, id: SmallUid, message: String) -> Self {
        MessageDB {
            board_id,
            id,
            message,
        }
    }
}

impl Board {
    pub fn new(name: String, messages: Vec<Message>) -> Result<Self> {
        Ok(Self {
            user: String::from(SmallUid::new()?),
            id: String::from(SmallUid::new()?),
            name,
            messages,
        })
    }

    pub fn compose(user: String, name: String, id: String, messages: Vec<Message>) -> Self {
        Self {
            user,
            id,
            name,
            messages,
        }
    }
}

impl TryFrom<Board> for BoardDB {
    type Error = Error;
    fn try_from(value: Board) -> Result<Self, Self::Error> {
        let user: SmallUid = value.user.try_into()?;
        let id: SmallUid = value.id.try_into()?;
        Ok(BoardDB {
            user,
            id,
            name: value.name,
        })
    }
}

impl BoardDB {
    pub fn new(name: String) -> Result<Self> {
        let user = SmallUid::new()?;
        let id = SmallUid::new()?;
        Ok(BoardDB { user, id, name })
    }

    pub fn compose(user: SmallUid, id: SmallUid, name: String) -> Self {
        BoardDB { user, id, name }
    }

    pub fn to_board(&self, messages: Vec<Message>) -> Board {
        Board {
            user: self.user.into(),
            id: self.id.into(),
            name: self.name.clone(),
            messages,
        }
    }
}
