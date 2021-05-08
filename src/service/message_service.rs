use mongodb::Collection;

use crate::models::message::Message;

#[derive(Clone)]
pub struct MessageService {
    collection: Collection<Message>,
}

impl MessageService {
    pub fn new(collection: Collection<Message>) -> MessageService {
        MessageService {
            collection: collection,
        }
    }
}
