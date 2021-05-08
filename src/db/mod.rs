use self::{message_db::MessageDB, user_db::UserDB};

pub(crate) mod message_db;
pub(crate) mod user_db;
#[derive(Clone)]
pub struct DBService {
    pub user_db: UserDB,
    pub message_db: MessageDB,
}

impl DBService {
    pub fn new(user_db: UserDB, message_db: MessageDB) -> DBService {
        DBService {
            message_db,
            user_db,
        }
    }
}
