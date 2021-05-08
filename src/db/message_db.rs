use bson::{doc, Document};
use futures::StreamExt;
use futures::TryStreamExt;
use mongodb::Collection;

use crate::models::message::Message;

#[derive(Clone)]
pub struct MessageDB {
    collection: Collection<Message>,
}

impl MessageDB {
    pub fn new(collection: Collection<Message>) -> MessageDB {
        MessageDB {
            collection: collection,
        }
    }

    pub async fn get_messages_between_two_users(
        &self,
        user1: String,
        user2: String,
    ) -> Option<Vec<Message>> {
        let query = doc! {
            "$or": [
                { "sentBy": user1.clone(), "recipient": user2.clone() },
                { "sentBy": user2, "recipient": user1 },
            ],
        };

        let documents = self.collection.find(query, None).await.ok();
        match documents {
            Some(docs) => {
                let messages: Vec<Message> = docs.try_collect().await.unwrap();
                Some(messages)
            }
            None => None,
        }
    }
}
