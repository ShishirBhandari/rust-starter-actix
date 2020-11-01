use dotenv::dotenv;
use actix_web::{App, HttpServer};
use std::env;
use service::user_service::UserService;
use service_manager::ServiceManager;

mod db;
mod controllers;
mod service;
mod dao;
mod entity;
mod service_manager;

struct AppState {
    service_manager: ServiceManager
}

#[actix_rt::main]
async fn main()-> std::io::Result<()> {
    dotenv().ok();

    let server_url = env::var("HOST").expect("SERVER_URL Environment variable is not set.");
    let database = db::create_connection();

    // let user_collection = database.collection("User");
    
    HttpServer::new(move || {
        let user_service_worker = UserService::new(database.clone());
        let service_manager = ServiceManager::new(user_service_worker);
        App::new()
        .data(AppState {service_manager})
        .configure(controllers::user_controller::init)

    })
    .bind(server_url)?
    .run()
    .await
}
