use crate::{
    models::{CreateStoreDto, UpdateStoreDto},
    repos::{pagination::PaginationDto, store_repo},
    routes::SearchBy,
    utils::{json_error_handler, AppData},
};
use actix_web::{
    get, post, put,
    web::{self, Data, ServiceConfig},
    HttpResponse,
};
use actix_web_validator::{Json, JsonConfig, Query};

#[get("{store_id}")]
async fn get(app_data: Data<AppData>, store_id: web::Path<i32>) -> HttpResponse {
    match app_data.pg_pool.get() {
        Ok(conn) => store_repo::get_store(conn, store_id.into_inner()).await,
        _ => HttpResponse::InternalServerError().body("Internal Server Error"),
    }
}

#[get("")]
async fn get_many(
    app_data: Data<AppData>,
    pagination: Query<PaginationDto>,
    search_by: Query<SearchBy>,
) -> HttpResponse {
    match app_data.pg_pool.get() {
        Ok(conn) => {
            store_repo::get_many(conn, pagination.into_inner(), search_by.into_inner()).await
        }
        _ => HttpResponse::InternalServerError().body("Internal Server Error"),
    }
}

#[post("")]
async fn post(app_data: Data<AppData>, store: Json<CreateStoreDto>) -> HttpResponse {
    match app_data.pg_pool.get() {
        Ok(conn) => store_repo::create_store(conn, store.into_inner()).await,
        Err(_) => HttpResponse::InternalServerError().body("Internal Server Error"),
    }
}

#[put("{id}")]
async fn update(
    app_data: Data<AppData>,
    store_id: web::Path<i32>,
    store: Json<UpdateStoreDto>,
) -> HttpResponse {
    match app_data.pg_pool.get() {
        Ok(conn) => store_repo::update_store(conn, store_id.into_inner(), store.into_inner()).await,
        _ => HttpResponse::InternalServerError().body("InternalServerError"),
    }
}

#[get("{store_id}")]
async fn delete(app_data: Data<AppData>, store_id: web::Path<i32>) -> HttpResponse {
    match app_data.pg_pool.get() {
        Ok(conn) => store_repo::delete_store(conn, store_id.into_inner()).await,
        _ => HttpResponse::InternalServerError().body("Internal Server Error"),
    }
}
pub fn init_store_routes(cfg: &mut ServiceConfig) {
    cfg.app_data(JsonConfig::default().error_handler(json_error_handler));
    cfg.service(get);
    cfg.service(get_many);
    cfg.service(post);
    cfg.service(update);
    cfg.service(delete);
}
