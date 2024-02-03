#[macro_use]
extern crate log;
#[macro_use(concat_string)]
extern crate concat_string;

use actix::Actor;
use actix_cors::Cors;
use dotenv::dotenv;
use env_logger;

pub mod routes;
mod errors;

use actix_web::{
    HttpServer,
    App,
    middleware::{
        Compress, 
        Logger, 
    },
    web,
    http,
};

use actix_files::Files;
pub mod models;
pub mod websocket;
use std::sync::{Arc, Mutex};

#[macro_use]
mod utils;
#[macro_use]
mod views;

//use crate::views::not_found;
use crate::routes::routes;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init_from_env(env_logger::Env::default().default_filter_or("debug"));
    let server = websocket::Server::new().start();

    HttpServer::new(move || {
        let _files = Files::new("/static", "static/").show_files_listing();

        App::new()
            .wrap(Compress::default())
            .data(server.clone())
            .service(_files)
            .configure(routes)
            //.default_service(web::route().to(crate::views::login_page))
    })

    //.bind("69.167.186.207:9285")?       // порт для разработки
    .bind("69.167.186.70:9280")?     // порт для автоматической доставки
    .run()
    .await
}
