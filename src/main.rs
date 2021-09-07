use actix_web::{web, App, HttpServer};
use std::env;

use std::io::Result;

use mongodb::{options::ClientOptions, Client};
use std::sync::*;

use actix_cors::Cors;
use actix_web::http::header;


mod logs_handlers;


#[actix_web::main]
async fn main() -> Result<()> {
    env::set_var("RUST_LOG", "actix_web=debug");
    let mut client_options = ClientOptions::parse("mongodb://localhost:27017").await.unwrap();
    client_options.app_name = Some("PlantApi".to_string());
    let client = web::Data::new(Mutex::new(Client::with_options(client_options).unwrap()));
    
    // Access-Control-Allow-Origin
    // Cors::new().supports_credentials() 
    //POST, GET, PUT, HEAD, DELETE, TRACE, CONNECT, PATCH, OPTIONS
    HttpServer::new(move || {
        let cors = Cors::default()
        .allowed_origin("http://localhost:3000")
        .allowed_methods(vec!["GET", "POST", "DELETE", "PUT"])
        .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
        .allowed_header(header::CONTENT_TYPE)
        .max_age(3600);

        App::new()
        .wrap(cors)    
        .app_data(client.clone())
        .service(web::scope("/api").configure(logs_handlers::scoped_config))
    })
    .bind("127.0.0.1:8081")?
    .run()
    .await
}

