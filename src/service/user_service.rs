use crate::{
    db::DBService,
    models::{
        message::Message,
        user::{Claims, PersonalApplicationUser, PublicFacingUser},
    },
    utils::UtilFunctions,
};
use jsonwebtoken::TokenData;

#[derive(Clone)]
pub struct UserService {
    DBService: DBService,
    util_functions: UtilFunctions,
}

impl UserService {
    pub fn new(DBService: DBService, util_functions: UtilFunctions) -> UserService {
        UserService {
            DBService: DBService,
            util_functions,
        }
    }

    pub async fn get_user_by_username(
        &self,
        username: String,
        token_data: TokenData<Claims>,
    ) -> Option<PublicFacingUser> {
        let user = self.DBService.user_db.get_user_by_username(username).await;

        match user {
            Some(user) => {
                self.util_functions
                    .make_public_facing_user(user, token_data)
                    .await
            }
            None => None,
        }
    }

    pub async fn strictly_get_user_by_username(&self, username: String) -> Option<bson::Document> {
        self.DBService.user_db.get_user_by_username(username).await
    }

    pub async fn get_personal_user(
        &self,
        token_data: TokenData<Claims>,
    ) -> Option<PersonalApplicationUser> {
        let user = self
            .DBService
            .user_db
            .get_user_by_id(token_data.claims.userId)
            .await?;
        self.util_functions
            .make_public_personal_facing_user(user)
            .await
    }

    pub async fn get_profile_picture(&self, username: String) -> Option<String> {
        Some(
            self.util_functions
                .get_profile_picture(self.strictly_get_user_by_username(username).await?),
        )
    }

    pub async fn get_messages_between_two_users(
        &self,
        user1: String,
        user2: String,
    ) -> Vec<Message> {
        let messages = self
            .DBService
            .message_db
            .get_messages_between_two_users(user1, user2)
            .await;

        match messages {
            Some(messages) => messages,
            None => vec![],
        }
    }
}
