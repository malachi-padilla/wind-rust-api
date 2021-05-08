use bson::{oid::ObjectId, Bson};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Message {
    sentBy: String,
    message: String,
    recipient: String,
    _id: ObjectId,
}
