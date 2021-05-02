use bson::Document;
use mongodb::Collection;

#[derive(Clone)]
pub struct MessageService {
    collection: Collection
}

impl MessageService {
    pub fn new(collection: Collection<Document>) -> MessageService {
        MessageService { collection: collection }
    }
}