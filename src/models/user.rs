use bson::Bson;
use chrono::{Date, DateTime, Utc};
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct User {
    _id: ObjectId,
    sentFriendRequests: Vec<String>,
    recievedFriendRequests: Vec<String>,
    friends: Vec<String>,
    lastOnline: DateTime<Utc>,
    username: String,
    password: String,
    createdAt: DateTime<Utc>,
    updatedAt: DateTime<Utc>,
    profilePicture: String,
    email: String,
}

#[derive(Serialize, Debug)]
pub struct PublicFacingUser {
    pub userId: String,
    pub relation: String,
    pub friends: Vec<Bson>,
    pub lastOnline: DateTime<Utc>,
    pub username: String,
    pub profilePicture: String,
}

#[derive(Serialize, Debug)]
pub struct PersonalApplicationUser {
    pub userId: String,
    pub sentFriendRequests: Vec<Bson>,
    pub recievedFriendRequests: Vec<Bson>,
    pub friends: Vec<Bson>,
    pub username: String,
    pub email: String,
    pub lastOnline: DateTime<Utc>,
    pub profilePicture: String,
}

#[derive(Deserialize, Debug)]
pub struct Claims {
    pub userId: String,
    pub iat: i32,
    pub exp: i32,
}
