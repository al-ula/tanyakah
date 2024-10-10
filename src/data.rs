use ulid::Ulid;

pub struct Board {
    board_id: Ulid,
    user_id: Ulid,
    messages: Vec<Message>,
}

impl Board {
    pub fn new(board_id: Ulid, user_id: Ulid, messages: Vec<Message>) -> Self {
        Self {
            board_id,
            user_id,
            messages,
        }
    }
}

pub struct Message {
    id: Ulid,
    message: String,
    replies: Vec<String>,
}

impl Message {
    pub fn new(id: Ulid, message: String, replies: Vec<String>) -> Self {
        Self {
            id,
            message,
            replies,
        }
    }
}

pub struct Boards {
    pub user_id: Ulid,
    pub board_id: Ulid,
}

pub struct Messages {
    pub board_id: Ulid,
    pub message: MessageEntry,
}

pub struct MessageEntry {
    pub id: Ulid,
    pub message: String,
}

pub struct Replies {
    pub message_id: Ulid,
    pub reply: String,
}
