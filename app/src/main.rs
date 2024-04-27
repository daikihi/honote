use actix_web::{get, web, App, HttpServer, Responder};
use models::honey::Honeies;

use crate::models::honey::Honey;

mod dao;
mod models;
mod service;
mod schemas;

#[get("/hello/{name}")]
async fn greet(name: web::Path<String>) -> impl Responder {
    format!("Hello {name}!")
}

#[get("/honeies/")]
async fn honey_print(pool: web::Data<Honeies>) -> impl Responder {
    let _all_honeies = pool.get_all_honeies();

    let i = 1i64;
    let id = models::honey::HoneyId { id: i };
    let name = models::honey::HoneyName {
        value: "hoge".to_string(),
    };
    let var_name = models::honey::Honey::new(id, name);
    let h = var_name;
    h.to_string()
}

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    let v = Vec::<Honey>::new();
    static honeies_status: Honeies = Honeies::new(v);

    HttpServer::new(|| App::new().service(greet).service(honey_print))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
