use jsonwebtoken::{TokenData};
use crate::{db::DB, models::user::{Claims, PersonalApplicationUser, PublicFacingUser}, utils::{UtilFunctions}};

#[derive(Clone)]
pub struct UserService {
    DB: DB,
    util_functions: UtilFunctions
}

impl UserService {
    pub fn new(db: DB, util_functions: UtilFunctions) -> UserService {
        UserService { DB: db, util_functions }
    }

    pub async fn get_user_by_username(&self, username: String, token_data: TokenData<Claims>) -> Option<PublicFacingUser> {
        let user = self.DB.get_user_by_username(username).await;

        match user {
            Some(user) => self.util_functions.make_public_facing_user(user, token_data).await,
            None => None
        }
    }


    pub async fn strictly_get_user_by_username(&self, username: String)  -> Option<bson::Document> {
        self.DB.get_user_by_username(username).await
    }

    pub async fn get_personal_user(&self, token_data: TokenData<Claims>) -> Option<PersonalApplicationUser> {
        let user = self.DB.get_user_by_id(token_data.claims.userId).await?;
        self.util_functions.make_public_personal_facing_user(user).await
    }

    pub async fn im_gay(self, username: String) -> Option<bson::Document> {
        self.DB.get_user_by_username(username).await
    }        // }

}