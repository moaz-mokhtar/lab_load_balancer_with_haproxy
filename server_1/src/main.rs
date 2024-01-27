mod handler;

use actix_cors::Cors;
use actix_web::{middleware::Logger, web, App, HttpServer};
use log::info;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Welcome to Server #1");
    initiate_logging();

    let server_address =
        std::env::var("SERVER_1_HOST").expect("Missed 'SERVER_1_HOST' environment variable");
    info!("Starting server at {}", server_address);

    HttpServer::new(move || {
        App::new()
            .wrap(Cors::permissive())
            .wrap(Logger::default())
            .service(web::scope("").configure(handler::routes_config))
    })
    .bind(server_address)?
    .run()
    .await
}

pub fn initiate_logging() {
    std::env::set_var("RUST_LOG", "debug, actix_web=debug");

    let env = dotenv::from_filename(".env").expect("'.env' not found.");
    dbg!(env);

    if std::env::var("PWD").is_err() {
        std::env::set_var("PWD", env!("CARGO_MANIFEST_DIR"));
        let pwd = std::env::var("PWD").unwrap();
        dbg!(pwd);
    }

    env_logger::init();
}
