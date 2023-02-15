use crate::{
    models::{CreateStoreDto, UpdateStoreDto, Store, QResult, PaginatedResult},
    repos::{pagination::PaginationDto, store_repo},
    routes::{validate_order, SearchBy},
    utils::{json_error_handler, AppData},
};
use actix_web::{
    get, post, put, delete,
    web::{self, Data, ServiceConfig},
    HttpResponse
};
use actix_web_validator::{Json, JsonConfig, Query, QueryConfig};
use chrono::{DateTime, Local, NaiveDateTime, Utc};
use serde::Deserialize;
use utoipa::{ToSchema, IntoParams};
use validator::{Validate, ValidationError};

use super::Stringify;

#[derive(Deserialize, Validate, Debug, ToSchema, IntoParams)]
pub struct DateFilter {
    #[schema(example = json!(Utc::now()))]
    pub before: Option<DateTime<Local>>,
    #[schema(example = json!(Utc::now()))]
    pub after: Option<DateTime<Local>>,
}

#[derive(Deserialize, Validate, Debug, ToSchema, IntoParams)]
pub struct StoresOrderBy {
    #[validate(custom = "validate_order")]
    #[schema(example = "ASC")]
    pub order: Option<String>,
    #[validate(custom = "validate_order_by")]
    #[schema(example = "prod_count")]
    pub by: Option<String>,
}

impl StoresOrderBy {
    pub fn option(self) -> Option<Self> {
        match self {
            Self {
                order: Some(x),
                by: Some(y),
            } => Some(Self {
                order: Some(x),
                by: Some(y),
            }),
            _ => None,
        }
    }
}

impl Stringify for Option<StoresOrderBy> {
    fn stringify(self) -> String {
        if let Some(StoresOrderBy {
            order: Some(order),
            by: Some(by),
        }) = self
        {
            format!("{} {}", by, order)
        } else {
            "created_at DESC".to_owned()
        }
    }
}

impl DateFilter {
    pub fn get_before(&self) -> NaiveDateTime {
        match self {
            DateFilter {
                before: Some(before_date),
                after: _,
            } => before_date.naive_utc(),
            _ => Utc::now().naive_utc(),
        }
    }

    pub fn get_after(&self) -> NaiveDateTime {
        match self {
            DateFilter {
                after: Some(after_date),
                before: _,
            } => after_date.naive_utc(),
            _ => DateTime::<Local>::default().naive_utc(),
        }
    }
}

/// Returns corresponding store with id=:store_id
#[utoipa::path(
    get, 
    path = "/store/{store_id}",
    params(
        ("store_id", description = "Unique id of stores")
    ),
    responses(
        (status = 200, description = "Returns the store with the corresponding id", body = QResult<Store>, example = json!(QResult {
            rows: Store { id: 1, name: "Store 1".to_owned(), is_holiday: false, created_at: Utc::now().naive_utc(), prod_count: 2 },
            error: None
        })),
    )
)]
#[get("{store_id}")]
async fn get(app_data: Data<AppData>, store_id: web::Path<i32>) -> HttpResponse {
    match app_data.pg_pool.get() {
        Ok(conn) => store_repo::get_store(conn, store_id.into_inner()).await,
        _ => HttpResponse::InternalServerError().body("Internal Server Error"),
    }
}

/// Returns a paginated list of stores
#[utoipa::path(
    get, 
    path = "/store",
    params(
        PaginationDto,
        StoresOrderBy,
        SearchBy,
        DateFilter
    ),
    responses(
        (status = 200, description = "Returns a list of products", body = PaginatedResult<Product>, example = json!(PaginatedResult {
            per_page: 10,
            page: 1,
            total_pages: 1,
            result: vec![
                Store { id: 1, name: "Store 1".to_owned(), is_holiday: false, created_at: Utc::now().naive_utc(), prod_count: 2 },
                Store { id: 2, name: "Store 2".to_owned(), is_holiday: false, created_at: Utc::now().naive_utc(), prod_count: 0 }
            ]
        })),
    )
)]
#[get("")]
async fn get_many(
    app_data: Data<AppData>,
    order: Query<StoresOrderBy>,
    pagination: Query<PaginationDto>,
    search_by: Query<SearchBy>,
    date: Query<DateFilter>,
) -> HttpResponse {
    match app_data.pg_pool.get() {
        Ok(conn) => {
            store_repo::get_many(
                conn,
                pagination.into_inner(),
                order.into_inner().option(),
                search_by.into_inner(),
                date.into_inner(),
            )
            .await
        }
        _ => HttpResponse::InternalServerError().body("Internal Server Error"),
    }
}

/// Creates a new store
#[utoipa::path(
    post, 
    path = "/store",
    request_body = CreateStoreDto,
    responses(
        (status = 200, body = QResult<Store>, example = json!(QResult {
            rows: Store { id: 2, name: "Store 2".to_owned(), is_holiday: false, created_at: Utc::now().naive_utc(), prod_count: 0 },
            error: None
        })),
    )
)]
#[post("")]
async fn post(app_data: Data<AppData>, store: Json<CreateStoreDto>) -> HttpResponse {
    match app_data.pg_pool.get() {
        Ok(conn) => store_repo::create_store(conn, store.into_inner()).await,
        Err(_) => HttpResponse::InternalServerError().body("Internal Server Error"),
    }
}

/// Edits store with corresponding ID
#[utoipa::path(
    put, 
    path = "/store/{id}",
    request_body = UpdateStoreDto,
    params (
        ("id", description = "id of store")
    ),
    responses(
        (status = 200, body = QResult<Store>, example = json!(QResult {
            rows: Store { id: 2, name: "Store 2".to_owned(), is_holiday: false, created_at: Utc::now().naive_utc(), prod_count: 0 },
            error: None
        })),
    )
)]
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

/// Deletes store with corresponding id
#[utoipa::path(
    delete, 
    path = "/store/{id}",
    params(
        ("id", description = "Unique id of store")
    ),
    responses(
        (status = 200, description = "return deleted store", body = QResult<Store>, example = json!(QResult {
            rows: Store { id: 2, name: "Store 2".to_owned(), is_holiday: false, created_at: Utc::now().naive_utc(), prod_count: 0 },
            error: None
        })),
    )
)]
#[delete("{store_id}")]
async fn delete(app_data: Data<AppData>, store_id: web::Path<i32>) -> HttpResponse {
    match app_data.pg_pool.get() {
        Ok(conn) => store_repo::delete_store(conn, store_id.into_inner()).await,
        _ => HttpResponse::InternalServerError().body("Internal Server Error"),
    }
}

/// get stores product count (never used in application)
#[utoipa::path(
    get, 
    path = "/store/{id}/count",
    params(
        ("id", description = "Unique id of store")
    ),
    responses(
        (status = 200, description = "return store with corresponding id products count", body = QResult<i32>, example = json!(QResult {
            rows: 22,
            error: None
        })),
    )
)]
#[get("{store_id}/count")]
async fn product_count(app_data: Data<AppData>, store_id: web::Path<i32>) -> HttpResponse {
    match app_data.pg_pool.get() {
        Ok(conn) => store_repo::product_count(conn, store_id.into_inner()).await,
        _ => HttpResponse::InternalServerError().body("Internal Server Error"),
    }
}

pub fn init_store_routes(cfg: &mut ServiceConfig) {
    cfg.app_data(JsonConfig::default().error_handler(json_error_handler));
    cfg.app_data(QueryConfig::default().error_handler(json_error_handler));
    cfg.service(get);
    cfg.service(get_many);
    cfg.service(post);
    cfg.service(update);
    cfg.service(delete);
    cfg.service(product_count);
}

fn validate_order_by(order_by: &str) -> Result<(), ValidationError> {
    if order_by == "name"
        || order_by == "id"
        || order_by == "created_at"
        || order_by == "prod_count"
    {
        return Ok(());
    }
    Err(ValidationError::new("invalid order by column"))
}
