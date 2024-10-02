use actix_web::{web, App, HttpServer};
use diesel::r2d2::{self, ConnectionManager};
use diesel::PgConnection;
use routers::*;

mod api;
mod dao;
mod models;
mod routers;
mod schema;
use log4rs;

#[macro_use]
extern crate diesel;

pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    log4rs::init_file("./config/log4rs.yaml", Default::default()).unwrap();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let database_pool = Pool::builder()
        .build(ConnectionManager::<PgConnection>::new(database_url))
        .expect("Failed to create pool.");

    let http_server = HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(database_pool.clone()))
            .configure(monster_routes)
    })
    .bind(("0.0.0.0", 9876))?
    .run();

    http_server.await
}
