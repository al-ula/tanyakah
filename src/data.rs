use ulid::Ulid;

pub struct Board {
    pub user_id: Ulid,
    pub board_id: Ulid,
    pub user_name: String,
}

impl Board {
    pub fn new(user_id: Ulid, user_name: String) -> Self {
        Self {
            user_id,
            board_id: Ulid::new(),
            user_name
        }
    }
}

pub struct Message {
    pub message_id: Ulid,
    pub board_id: Ulid,
    pub content: String,
}

impl Message {
    pub fn new(board_id: Ulid, content: String) -> Self {
        Self {
            message_id: Ulid::new(),
            board_id,
            content
        }
    }
}

pub struct Reply {
    pub reply_id: Ulid,
    pub message_id: Ulid,
    pub content: String,
}

impl Reply {
    pub fn new(message_id: Ulid, content: String) -> Self {
        Self {
            reply_id: Ulid::new(),
            message_id,
            content
        }
    }
}

pub fn register(user_name: String) -> Board {
    Board::new(Ulid::new(), user_name)
}