use bson::{doc, oid::ObjectId};
use mongodb::Cursor;
use mongodb::{bson::Document, results::InsertOneResult, Collection};

use crate::controller::RegisterUserRequest;

#[derive(Clone)]
pub struct UserDB {
    collection: Collection,
}

impl UserDB {
    pub fn new(collection: Collection) -> UserDB {
        UserDB {
            collection: collection,
        }
    }

    pub async fn get_all_users(&self) -> Cursor<Document> {
        self.collection.find(None, None).await.unwrap()
    }

    pub async fn get_user_by_username(&self, username: String) -> Option<bson::Document> {
        self.collection
            .find_one(doc! { "username": username }, None)
            .await
            .unwrap()
    }

    pub async fn get_user_by_id(&self, id: String) -> Option<bson::Document> {
        self.collection
            .find_one(doc! { "_id": ObjectId::with_string(&id).unwrap() }, None)
            .await
            .unwrap()
    }

    pub async fn insert_user(&self, register_request: RegisterUserRequest) -> InsertOneResult {
        let user = doc! {
        "username": register_request.username,
        "password": register_request.password,
        "email": register_request.email,
        "sentFriendRequests": [],
        "recievedFriendRequests": [],
        "friends": [],
        "lastOnline": "",
        "profilePicture": ""
        };
        self.collection.insert_one(user, None).await.unwrap()
    }
}
