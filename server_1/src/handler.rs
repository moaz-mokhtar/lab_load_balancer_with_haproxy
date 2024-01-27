use actix_web::{
    web::{self, ServiceConfig},
    HttpRequest, HttpResponse, Responder,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct MessageResponse {
    pub status: String,
    pub message: String,
}

pub fn routes_config(config: &mut ServiceConfig) {
    config
        .service(web::resource("/").route(web::get().to(health)))
        .service(web::resource("/connect").route(web::get().to(connect)));
}
const SERVER_NAME: &str = "Server 1";

async fn health() -> impl Responder {
    let message = format!(
        "\t - Status: Healthy - {} - Time: {}",
        SERVER_NAME,
        chrono::Local::now()
    );
    println!("{}", message);
    HttpResponse::Ok().body(message)
}

async fn connect(req: HttpRequest) -> impl Responder {
    let message = format!(
        "\t -- Connect?: Connected to {} - Time: {} ]===",
        SERVER_NAME,
        chrono::Local::now().format("%Y-%m-%d %H:%M:%S:%.3f")
    );
    println!("{}", message);
    println!("\t --- Http Request: {:?}", req);

    HttpResponse::Ok().body(message)
}
