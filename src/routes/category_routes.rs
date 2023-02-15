use crate::{
    models::{CategoryDto, UpdateCategoryDto, Category, PaginatedResult, QResult},
    repos::{category_repo, pagination::PaginationDto},
    routes::OrderBy,
    utils::{json_error_handler, AppData},
    
};
use actix_web::{delete, get, post, put, web, HttpResponse};
use actix_web_validator::{Json, JsonConfig, Query, QueryConfig};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use super::SearchBy;

#[derive(Deserialize, Serialize, ToSchema)]
pub struct ManyIdsDto {
    #[schema(example = json!([1, 2, 3]))]
    ids: Vec<i32>,
}

/// Fetches category with corresponding ID
#[utoipa::path(
    get, 
    path = "/category/{id}",
    params(
        ("id", description = "Unique id of categories")
    ),
    responses(
        (status = 200, description = "Returns the category with the id", body = QResult<Category>, example = json!(QResult {
            rows: Category { id: 1, name: "My category".to_owned(), created_at: Utc::now().naive_utc()},
            error: None
        })),
    )
)]
#[get("{id}")]
async fn get(app_data: web::Data<AppData>, id: web::Path<i32>) -> HttpResponse {
    match app_data.pg_pool.get() {
        Ok(conn) => category_repo::get_category(conn, id.into_inner()).await,
        _ => HttpResponse::InternalServerError().body("Internal Server Error"),
    }
}

/// Fetches categories with corresponding ID
#[utoipa::path(
    get, 
    path = "/category",
    params(
        PaginationDto,
        OrderBy,
        SearchBy,
    ),
    responses(
        (status = 200, description = "Returns a list of categories", body = PaginatedResult<Category>, example = json!(PaginatedResult {
            per_page: 10,
            page: 1,
            total_pages: 1,
            result: vec![
                Category {id: 1, name: "Category 1".to_owned(), created_at: Utc::now().naive_utc()}, 
                Category {id: 2, name: "Category 2".to_owned(), created_at: Utc::now().naive_utc()}
            ]
        })),
    )
)]
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

/// Creates a new Category
#[utoipa::path(
    post, 
    path = "/category",
    request_body = CategoryDto,
    responses(
        (status = 200, body = QResult<Category>, example = json!(QResult {
            rows: Category {id: 2, name: "Category 2".to_owned(), created_at: Utc::now().naive_utc()},
            error: None
        })),
    )
)]
#[post("")]
async fn post(app_data: web::Data<AppData>, category: Json<CategoryDto>) -> HttpResponse {
    match app_data.pg_pool.get() {
        Ok(conn) => category_repo::add_category(conn, category.into_inner()).await,
        _ => HttpResponse::InternalServerError().body("Internal Server Error"),
    }
}

/// Updates category with corresponding ID
#[utoipa::path(
    put, 
    path = "/category",
    request_body = UpdateCategoryDto,
    responses(
        (status = 200, body = QResult<Category>, example = json!(QResult {
            rows: Category {id: 2, name: "Category 2".to_owned(), created_at: Utc::now().naive_utc()},
            error: None
        })),
    )
)]
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

/// Deletes category with corresponding ID
#[utoipa::path(
    delete, 
    path = "/category/{id}",
    params(
        ("id", description = "Unique id of categories")
    ),
    responses(
        (status = 200, description = "returns deleted product", body = QResult<Category>, example = json!(QResult {
            rows: Category {id: 2, name: "Category 2".to_owned(), created_at: Utc::now().naive_utc()},
            error: None
        })),
    )
)]
#[delete("{id}")]
async fn delete(app_data: web::Data<AppData>, cat_id: web::Path<i32>) -> HttpResponse {
    match app_data.pg_pool.get() {
        Ok(conn) => category_repo::delete_category(conn, cat_id.into_inner()).await,
        _ => HttpResponse::InternalServerError().body("Internal Server Error"),
    }
}


/// Deletes cateogories with corresponding IDs
#[utoipa::path(
    delete, 
    path = "/category",
    request_body = ManyIdsDto,
    responses(
        (status = 200, body = QResult<Category>, example = json!(QResult {
            rows: Category {id: 2, name: "Category 2".to_owned(), created_at: Utc::now().naive_utc()},
            error: None
        })),
    )
)]
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
