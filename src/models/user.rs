use mongodb::bson::{DateTime, oid::ObjectId};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct User {
    _id: ObjectId,
    sentFriendRequests: Vec<String>,
    recievedFriendRequests: Vec<String>,
    friends: Vec<String>,
    lastOnline: DateTime,
    username: String,
    password: String,    
    createdAt: DateTime,
    updatedAt: DateTime,
    profilePicture: String,
    email: String
}