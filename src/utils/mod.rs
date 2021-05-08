#![feature(async_closure)]

use crate::{
    db::DBService,
    models::user::{Claims, PersonalApplicationUser, PublicFacingUser},
};
use bson::{Bson, Document};
use jsonwebtoken::{Algorithm, DecodingKey, TokenData, Validation};

#[derive(Clone)]
pub struct UtilFunctions {
    DBService: DBService,
}

impl UtilFunctions {
    pub fn new(DBService: DBService) -> UtilFunctions {
        UtilFunctions { DBService }
    }

    pub async fn make_public_facing_user(
        &self,
        doc: Document,
        token_data: TokenData<Claims>,
    ) -> Option<PublicFacingUser> {
        let user_id_string = doc.get("_id")?.as_object_id()?.to_hex();
        let user_id: Bson = user_id_string.clone().into();

        let mut relation = "None".to_string();
        let my_db_user = self
            .DBService
            .user_db
            .get_user_by_id(token_data.claims.userId)
            .await?;
        let sentFriendRequests = my_db_user.get_array("sentFriendRequests").ok()?;
        let recievedFriendRequests = my_db_user.get_array("recievedFriendRequests").ok()?;
        let friends = my_db_user.get_array("friends").ok()?;

        if sentFriendRequests.contains(&user_id) {
            relation = "Requested".to_string();
        } else if recievedFriendRequests.contains(&user_id) {
            relation = "Recipient Requested".to_string();
        } else if friends.contains(&user_id) {
            relation = "Friends".to_string();
        }
        let profile_picture = self.get_profile_picture(doc.clone());
        let user = PublicFacingUser {
            userId: user_id_string,
            friends: doc.get("friends")?.as_array()?.to_owned(),
            lastOnline: *doc.get("lastOnline")?.as_datetime()?,
            username: doc.get("username")?.as_str()?.to_owned(),
            relation: relation,
            profilePicture: profile_picture,
        };
        Some(user)
    }
    pub async fn make_public_personal_facing_user(
        &self,
        doc: Document,
    ) -> Option<PersonalApplicationUser> {
        let user = PersonalApplicationUser {
            userId: doc.get("_id")?.as_object_id()?.to_hex(),
            friends: doc.get("friends")?.as_array()?.to_owned(),
            lastOnline: *doc.get("lastOnline")?.as_datetime()?,
            username: doc.get("username")?.as_str()?.to_owned(),
            email: doc.get("email")?.as_str()?.to_owned(),
            recievedFriendRequests: doc.get("recievedFriendRequests")?.as_array()?.to_owned(),
            sentFriendRequests: doc.get("recievedFriendRequests")?.as_array()?.to_owned(),
            profilePicture: self.get_profile_picture(doc),
        };
        Some(user)
    }

    pub fn get_profile_picture(&self, user: Document) -> String {
        if let Some(title) = user.get("profilePicture").and_then(Bson::as_str) {
            if title == "" {
                "https://source.unsplash.com/random".to_string()
            } else {
                format!(
                    "https://wind-profile-pictures.s3-us-west-1.amazonaws.com/{}",
                    title
                )
                .to_string()
            }
        } else {
            "https://source.unsplash.com/random".to_string()
        }
    }
}

pub fn verify_jwt(token: &str) -> Result<TokenData<Claims>, jsonwebtoken::errors::Error> {
    jsonwebtoken::decode::<Claims>(
        token,
        &DecodingKey::from_secret("mysecretsigningkey".as_ref()),
        &Validation::new(Algorithm::HS256),
    )
}
