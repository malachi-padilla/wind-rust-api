use std::fmt::format;

use actix_web::web::{Data, Json, Query};
use futures::stream::StreamExt;
use actix_web::{HttpResponse, get, post};
use bson::Bson;
use serde::{Deserialize};
use crate::{models::user::User, service::user_service::UserService};

#[get("/")]
pub async fn get_all_users(data: Data<UserService>) -> HttpResponse {
    let mut users = data.get_all_users().await;
    let mut docs = Vec::new();

    while let Some(result) = users.next().await {
        match result {
            Ok(document) => {
                let message: User = bson::from_bson(Bson::Document(document)).unwrap();
                docs.push(message);
            }
            Err(_) => println!("Error"),
        }
    }
    
    HttpResponse::Ok().json(docs)
}

#[derive(Deserialize)]
pub struct GetUserByUsernameRequest {
    username: String
}

#[derive(Deserialize)]
pub struct GetUserByIDRequest {
    id: String
}

#[derive(Deserialize)]
pub struct RegisterUserRequest {
    pub username: String,
    pub password: String,
    pub email: String
}

pub struct GetProfilePictureResponse {

}


#[get("/getUserById")]
pub async fn get_user_by_id(data: Data<UserService>, params: Query<GetUserByUsernameRequest>) -> HttpResponse {
    match data.get_user_by_username(params.username.clone()).await {
        Some(user) => HttpResponse::Ok().json(user),
        None => HttpResponse::Ok().json("No User Found")
    }
}

#[post("/registerUser")]
pub async fn register_user(data: Data<UserService>, request_body: Json<RegisterUserRequest>) -> HttpResponse {
    data.insert_user(request_body.0).await;
    HttpResponse::Ok().json("Success")
}

#[get("/getProfilePicture")]
pub async fn get_profile_picture(data: Data<UserService>, params: Query<GetUserByUsernameRequest>) -> HttpResponse {
    match data.get_user_by_username(params.username.clone()).await {
        Some(user) => {
            if let Some(title) = user.get("profilePicture").and_then(Bson::as_str) {
                if title == "" {
                    HttpResponse::Ok().json("https://source.unsplash.com/random")
                } else {
                    HttpResponse::Ok().json(format!("https://wind-profile-pictures.s3-us-west-1.amazonaws.com/{}", title))
                }
            } else {
                HttpResponse::Ok().json("No Picture Found")
            }
        },
        None => HttpResponse::Ok().json("No User Found")
    }
}


// router.get("/getProfilePicture", async (req, res) => {
//     try {
//       const { userId, username } = req.query;
  
//       let user: any;
//       if (username) {
//         user = await User.findOne({ username });
//       } else if (userId) {
//         user = await User.findById(userId);
//       }
  
//       const profilePicture = user.profilePicture;
  
//       if (profilePicture && profilePicture !== "") {
//         generateS3BucketUrl(process.env.PROFILE_PICTURES_BUCKET, profilePicture)
//           .then((url) => {
//             res.send(url);
//           })
//           .catch((err) => {
//             console.log(err);
//             res.status(400).send("Error");
//           });
//       } else {
//         res.send("https://source.unsplash.com/random");
//       }
//     } catch (e) {
//       res.status(400).send("Error fetching user");
//     }
//   });