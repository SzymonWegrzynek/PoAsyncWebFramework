use actix_web::{middleware::Logger, web, App, HttpServer};
use dotenv::dotenv;
use std::env;

mod handlers;
mod models;
mod modules;
mod routes;
mod state;
mod tests;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    env_logger::init();

    let app_state = state::AppState {};

    let server_ip = env::var("SERVER_IP").expect("SERVER_IP must be set");

    let server_port = env::var("SERVER_PORT")
        .expect("SERVER_PORT must be set")
        .parse::<u16>()
        .expect("Invalid port number");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(app_state.clone()))
            .wrap(Logger::default())
            .configure(routes::routes::healthcheck)
    })
    .bind((server_ip, server_port))?
    .run()
    .await?;

    Ok(())
}
