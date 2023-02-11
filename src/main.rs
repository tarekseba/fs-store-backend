use crate::{
    config::Config,
    routes::{init_category_routes, init_product_routes, init_store_routes},
    utils::{create_conn_pool, server_running, AppData},
};
use actix_cors::Cors;
use actix_web::{self, main, web, App, HttpServer};

mod config;
mod models;
mod repos;
mod routes;
mod schema;
mod utils;

#[main]
async fn main() -> std::io::Result<()> {
    let config: Config = Config::load_config();
    println!("before");
    let app_data: AppData = create_conn_pool(&config);
    println!("after");
    server_running(&config);
    HttpServer::new(move || {
        let cors = Cors::permissive();
        App::new()
            .wrap(cors)
            .app_data(web::Data::new(app_data.clone()))
            .service(web::scope("/category").configure(init_category_routes))
            .service(web::scope("/product").configure(init_product_routes))
            .service(web::scope("/store").configure(init_store_routes))
    })
    .bind((config.get_srv_addr(), *config.get_srv_port()))?
    .run()
    .await
}
