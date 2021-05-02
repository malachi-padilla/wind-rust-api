#![feature(proc_macro_hygiene, decl_macro)]

extern crate mongodb;
extern crate actix_web;

mod models;
mod service;
mod controller;
use actix_cors::Cors;
use bson::Document;
use actix_web::{App, HttpServer};
use mongodb::{Client, Collection, options::ClientOptions};
use service::{user_service::UserService, message_service::MessageService};
use controller::{get_all_users, get_profile_picture, get_user_by_id, register_user};

#[actix_web::main]
async fn main() -> std::io::Result<()> {


    let client_options = ClientOptions::parse("mongodb+srv://malachi:123@cluster0.npkqi.mongodb.net/users?retryWrites=true&w=majority").await.unwrap();
    let client = Client::with_options(client_options).unwrap();
    let db = client.database("users");
    
    let user_collection = db.collection("users");
    let message_collection: Collection<Document> = db.collection("messages");

    let user_service = UserService::new(user_collection);
    let message_service = MessageService::new(message_collection);


    

    HttpServer::new(move || {
        let cors = Cors::permissive()
            .allowed_origin("http://localhost:3000")
            .allowed_methods(vec!["POST", "PUT", "PATCH", "GET", "OPTIONS", "HEAD"]);
            
        App::new()
            .data(user_service.clone())
            .data(message_service.clone())
            .wrap(cors)
            .service(get_all_users)
            .service(get_user_by_id)
            .service(register_user)
            .service(get_profile_picture)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
