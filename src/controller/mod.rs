use actix_web::{HttpMessage, HttpRequest, web::{Data, Query}};
use actix_web::{HttpResponse, get};
use bson::Bson;
use jsonwebtoken::errors::ErrorKind;
use serde::{Deserialize,Serialize};
use crate::{service::user_service::UserService, utils::verify_jwt};

// #[get("/getAllUsers")]
// pub async fn get_all_users(data: Data<UserService>) -> HttpResponse {
//     let mut users = data.get_all_users().await;
//     let mut docs = Vec::new();

//     while let Some(result) = users.next().await {
//         match result {
//             Ok(document) => {
//                 let message = bson::from_bson(Bson::Document(document)).unwrap();
//                 if let Some(user) = make_public_facing_user(message) {
//                     docs.push(user);
//                 }
//             }
//             Err(_) => println!("Error"),
//         }
//     }
    
//     HttpResponse::Ok().json(docs)
// }

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
#[derive(Serialize)]
pub struct GetProfilePicturesFromUsernamesResponse {
    username: String,
}

#[get("/getUserByUsername")]
pub async fn get_user_by_username(req: HttpRequest, data: Data<UserService>, params: Query<GetUserByUsernameRequest>) -> HttpResponse {

    let cookies = req.cookies().unwrap();
    let token = cookies.iter().find(|item| {
        item.name() == "token"
    }).unwrap().value();

    let user_claims = verify_jwt(token);

    match user_claims {
        Ok(token_data) => {
            match data.get_user_by_username(params.username.clone(), token_data).await {
                Some(user) => HttpResponse::Ok().json(user),
                None => HttpResponse::Ok().json("No User Found")
            }   
        },
        Err(error) => {
            match error.kind() {
                ErrorKind::ExpiredSignature => HttpResponse::BadRequest().json("Expired Token"),
                _ => HttpResponse::BadRequest().json("Invalid Token")
            }
        }
    } 
}

#[get("/getPersonalUser")]
pub async fn get_personal_user(req: HttpRequest, data: Data<UserService>) -> HttpResponse {
    let cookies = req.cookies().unwrap();
    let token = cookies.iter().find(|item| {
        item.name() == "token"
    }).unwrap().value();

    let user_claims = verify_jwt(token);
    match user_claims {
        Ok(token_data) => {
            let personal_user = data.get_personal_user(token_data).await;
            match personal_user {
                Some(user) => HttpResponse::Ok().json(user),
                None => HttpResponse::Ok().json("No User Found")
            }
        },
        Err(error) => {
            match error.kind() {
                ErrorKind::ExpiredSignature => HttpResponse::BadRequest().json("Expired Token"),
                _ => HttpResponse::BadRequest().json("Invalid Token")
            }
        }
    } 

}
// #[post("/registerUser")]
// pub async fn register_user(data: Data<UserService>, request_body: Json<RegisterUserRequest>) -> HttpResponse {
//     data.insert_user(request_body.0).await;
//     HttpResponse::Ok().json("Success")
// }

#[get("/getProfilePicture")]
pub async fn get_profile_picture(data: Data<UserService>, params: Query<GetUserByUsernameRequest>) -> HttpResponse {
    // revise)
    match data.strictly_get_user_by_username(params.username.clone()).await {
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