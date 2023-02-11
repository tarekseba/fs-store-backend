use crate::{
    models::{CategoryDto, UpdateCategoryDto},
    repos::{category_repo, pagination::PaginationDto},
    routes::OrderBy,
    utils::{json_error_handler, AppData},
};
use actix_web::{delete, get, post, put, web, HttpResponse};
use actix_web_validator::{Json, JsonConfig, Query, QueryConfig};
use serde::Deserialize;

use super::SearchBy;

#[derive(Deserialize)]
struct ManyIdsDto {
    ids: Vec<i32>,
}

#[get("{id}")]
async fn get(app_data: web::Data<AppData>, id: web::Path<i32>) -> HttpResponse {
    match app_data.pg_pool.get() {
        Ok(conn) => category_repo::get_category(conn, id.into_inner()).await,
        _ => HttpResponse::InternalServerError().body("Internal Server Error"),
    }
}

#[get("")]
async fn get_many(
    app_data: web::Data<AppData>,
    pagination: Query<PaginationDto>,
    order: Query<OrderBy>,
    search_by: Query<SearchBy>,
) -> HttpResponse {
    match app_data.pg_pool.get() {
        Ok(conn) => {
            category_repo::get_many(
                conn,
                pagination.into_inner(),
                order.into_inner().option(),
                search_by.into_inner(),
            )
            .await
        }
        _ => HttpResponse::InternalServerError().body("Internal Server Error"),
    }
}

#[post("")]
async fn post(app_data: web::Data<AppData>, category: Json<CategoryDto>) -> HttpResponse {
    match app_data.pg_pool.get() {
        Ok(conn) => category_repo::add_category(conn, category.into_inner()).await,
        _ => HttpResponse::InternalServerError().body("Internal Server Error"),
    }
}

#[put("{id}")]
async fn update(
    app_data: web::Data<AppData>,
    category: Json<UpdateCategoryDto>,
    cat_id: web::Path<i32>,
) -> HttpResponse {
    match app_data.pg_pool.get() {
        Ok(conn) => {
            category_repo::update_category(conn, category.into_inner(), cat_id.into_inner()).await
        }
        _ => HttpResponse::InternalServerError().body("Internal Server Error"),
    }
}

#[delete("{id}")]
async fn delete(app_data: web::Data<AppData>, cat_id: web::Path<i32>) -> HttpResponse {
    match app_data.pg_pool.get() {
        Ok(conn) => category_repo::delete_category(conn, cat_id.into_inner()).await,
        _ => HttpResponse::InternalServerError().body("Internal Server Error"),
    }
}

#[delete("")]
async fn delete_many(app_data: web::Data<AppData>, ids: web::Json<ManyIdsDto>) -> HttpResponse {
    match app_data.pg_pool.get() {
        Ok(conn) => category_repo::delete_many(conn, ids.into_inner().ids).await,
        _ => HttpResponse::InternalServerError().body("Internal Server Error"),
    }
}

pub fn init_category_routes(cfg: &mut web::ServiceConfig) {
    let json_cfg = JsonConfig::default().error_handler(json_error_handler);
    let query_cfg = QueryConfig::default().error_handler(json_error_handler);
    cfg.app_data(json_cfg);
    cfg.service(get);
    cfg.service(get_many).app_data(query_cfg);
    cfg.service(post);
    cfg.service(update);
    cfg.service(delete);
    cfg.service(delete_many);
}
