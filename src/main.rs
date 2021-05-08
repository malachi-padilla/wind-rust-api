extern crate actix_web;
extern crate mongodb;

mod controller;
mod db;
mod models;
mod service;
mod utils;
use actix_cors::Cors;
use actix_web::{App, HttpServer};
use bson::Document;
use controller::{get_personal_user, get_profile_picture, get_user_by_username};
use mongodb::{options::ClientOptions, Client, Collection};
use service::{message_service::MessageService, user_service::UserService};
use utils::UtilFunctions;

use crate::{
    controller::get_messages_between_two_users,
    db::{message_db::MessageDB, user_db::UserDB, DBService},
    models::message::Message,
};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let client_options = ClientOptions::parse(
        "mongodb+srv://malachi:123@cluster0.npkqi.mongodb.net/users?retryWrites=true&w=majority",
    )
    .await
    .unwrap();
    let client = Client::with_options(client_options).unwrap();
    let db = client.database("users");

    HttpServer::new(move || {
        let user_collection = db.collection("users");
        let message_collection = db.collection::<Message>("messages");

        let user_db = UserDB::new(user_collection.clone());
        let message_db = MessageDB::new(message_collection.clone());

        let db_service = DBService::new(user_db, message_db);
        let util_functions = UtilFunctions::new(db_service.clone());
        let user_service = UserService::new(db_service.clone(), util_functions.clone());
        let message_service = MessageService::new(message_collection.clone());

        let cors = Cors::permissive()
            .allowed_origin("http://localhost:3000")
            .allowed_methods(vec!["POST", "PUT", "PATCH", "GET", "OPTIONS", "HEAD"]);

        App::new()
            .data(user_service.clone())
            .data(message_service.clone())
            .wrap(cors)
            .service(get_user_by_username)
            .service(get_personal_user)
            .service(get_profile_picture)
            .service(get_messages_between_two_users)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
