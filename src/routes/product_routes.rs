use crate::{
    models::{ProductDto, UpdateProductDto, CategoryId},
    repos::{pagination::PaginationDto, product_repo},
    utils::{json_error_handler, AppData},
};
use actix_web::{
    delete, get, post, put,
    web::{self, ServiceConfig},
    HttpResponse,
};
use actix_web_validator::{Json, JsonConfig, Query, QueryConfig};
use serde::Deserialize;
use validator::{Validate, ValidationError};

fn validate_order(order: &str) -> Result<(), ValidationError> {
    if order == "ASC" || order == "DESC" {
        return Ok(());
    }
    Err(ValidationError::new("invalid order value"))
}

fn validate_order_by(order_by: &str) -> Result<(), ValidationError> {
    if order_by == "name"
        || order_by == "id"
        || order_by == "description"
        || order_by == "price"
        || order_by == "created_at"
    {
        return Ok(());
    }
    Err(ValidationError::new("invalid order by column"))
}
#[derive(Deserialize, Validate, Debug)]
pub struct OrderBy {
    #[validate(custom = "validate_order")]
    pub order: Option<String>,
    #[validate(custom = "validate_order_by")]
    pub by: Option<String>,
}

impl OrderBy {
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

#[derive(Deserialize, Validate, Debug)]
pub struct SearchBy {
    #[validate(length(max = 256))]
    pub name: Option<String>,
    #[validate(length(max = 256))]
    pub description: Option<String>,
    pub in_holiday: Option<bool>,
}

impl SearchBy {
    pub fn get_name(&self) -> String {
        if let Self {
            name: Some(name),
            description: _,
            in_holiday: _,
        } = self
        {
            format!("%{}%", name)
        } else {
            "%".to_owned()
        }
    }

    pub fn get_description(&self) -> String {
        if let Self {
            name: _,
            description: Some(description),
            in_holiday: _,
        } = self
        {
            format!("%{}%", description)
        } else {
            "%".to_owned()
        }
    }

    pub fn get_is_holiday(&self) -> &bool {
        if let Self {
            name: _,
            description: _,
            in_holiday: Some(val),
        } = self
        {
            val
        } else {
            &false
        }
    }

    pub fn get_is_holiday_neg(&self) -> &bool {
        if let Self {
            name: _,
            description: _,
            in_holiday: Some(val),
        } = self
        {
            val
        } else {
            &true
        }
    }
}

pub trait Stringify {
    fn stringify(self) -> String;
}

impl Stringify for Option<OrderBy> {
    fn stringify(self) -> String {
        if let Some(OrderBy {
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

#[get("{prod_id}")]
async fn get(app_data: web::Data<AppData>, prod_id: web::Path<i32>) -> HttpResponse {
    match app_data.pg_pool.get() {
        Ok(conn) => product_repo::get_product(conn, prod_id.into_inner()).await,
        _ => HttpResponse::InternalServerError().body("Internal Server Error"),
    }
}

#[get("")]
async fn get_many(
    app_data: web::Data<AppData>,
    pagination: Query<PaginationDto>,
    order: Query<OrderBy>,
    search: Query<SearchBy>,
    category_id: Query<CategoryId>,
) -> HttpResponse {
    match app_data.pg_pool.get() {
        Ok(conn) => {
            product_repo::get_many(
                conn,
                pagination.into_inner(),
                order.into_inner().option(),
                search.into_inner(),
                category_id.into_inner().category_id
            )
            .await
        }
        _ => HttpResponse::InternalServerError().body("Internal Server Error"),
    }
}

#[post("")]
async fn post(app_data: web::Data<AppData>, prod: Json<ProductDto>) -> HttpResponse {
    match app_data.pg_pool.get() {
        Ok(conn) => product_repo::add_product(conn, prod.into_inner()).await,
        _ => HttpResponse::InternalServerError().body("Internal Server Error"),
    }
}

#[put("{id}")]
async fn update(
    app_data: web::Data<AppData>,
    prod_id: web::Path<i32>,
    prod: Json<UpdateProductDto>,
) -> HttpResponse {
    match app_data.pg_pool.get() {
        Ok(conn) => {
            product_repo::update_product(conn, prod_id.into_inner(), prod.into_inner()).await
        }
        _ => HttpResponse::InternalServerError().body("Internal Server Error"),
    }
}

#[delete("{id}")]
async fn delete(app_data: web::Data<AppData>, id: web::Path<i32>) -> HttpResponse {
    match app_data.pg_pool.get() {
        Ok(conn) => product_repo::delete_product(conn, id.into_inner()).await,
        _ => HttpResponse::InternalServerError().body("Internal Server Error"),
    }
}

#[put("{prod_id}/category/{cat_id}")]
async fn attach_category(
    app_data: web::Data<AppData>,
    path: web::Path<(i32, i32)>,
) -> HttpResponse {
    println!("inside attach");
    match app_data.pg_pool.get() {
        Ok(conn) => product_repo::attach_category(conn, path.0, path.1).await,
        _ => HttpResponse::InternalServerError().body("Internal Server Error"),
    }
}

#[put("{prod_id}/store/{store_id}")]
async fn attach_store(app_data: web::Data<AppData>, path: web::Path<(i32, i32)>) -> HttpResponse {
    match app_data.pg_pool.get() {
        Ok(conn) => product_repo::attach_store(conn, path.0, path.1).await,
        _ => HttpResponse::InternalServerError().body("Internal Server Error"),
    }
}

#[delete("{prod_id}/category/{cat_id}")]
async fn dettach_category(
    app_data: web::Data<AppData>,
    path: web::Path<(i32, i32)>,
) -> HttpResponse {
    match app_data.pg_pool.get() {
        Ok(conn) => product_repo::dettach_category(conn, path.0, path.1).await,
        _ => HttpResponse::InternalServerError().body("Internal Server Error"),
    }
}

pub fn init_product_routes(cfg: &mut ServiceConfig) {
    cfg.app_data(JsonConfig::default().error_handler(json_error_handler));
    cfg.app_data(QueryConfig::default().error_handler(json_error_handler));
    cfg.service(get);
    cfg.service(get_many);
    cfg.service(post);
    cfg.service(update);
    cfg.service(delete);
    cfg.service(attach_category);
    cfg.service(dettach_category);
    cfg.service(attach_store);
}
