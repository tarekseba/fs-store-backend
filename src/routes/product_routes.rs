use crate::{
    models::{CategoryId, ProductDto, UpdateProductDto, StoreId, Product, PaginatedResult, QResult, ProductsCategories},
    repos::{pagination::PaginationDto, product_repo},
    utils::{json_error_handler, AppData},
};
use actix_web::{
    delete, get, post, put,
    web::{self, ServiceConfig},
    HttpResponse,
};
use actix_web_validator::{Json, JsonConfig, Query, QueryConfig};
use bigdecimal::BigDecimal;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use utoipa::{ToSchema, IntoParams};
use validator::{Validate, ValidationError};

pub fn validate_order(order: &str) -> Result<(), ValidationError> {
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
#[derive(Deserialize, Validate, Debug, ToSchema, IntoParams)]
pub struct OrderBy {
    #[validate(custom = "validate_order")]
    #[schema(example = "ASC")]
    pub order: Option<String>,
    #[validate(custom = "validate_order_by")]
    #[schema(example = "created_at")]
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

#[derive(Serialize, Deserialize, Validate, Debug, ToSchema, IntoParams)]
pub struct SearchBy {
    #[validate(length(max = 256))]
    #[schema(example = "substring to look for")]
    pub name: Option<String>,
    #[validate(length(max = 256))]
    #[schema(example = "substring to look for")]
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
// {
// 	"rows": {
// 		"id": 14,
// 		"name": "lol",
// 		"i18n_name": "haha",
// 		"price": "10.00",
// 		"description": "lol",
// 		"i18n_description": "lol",
// 		"created_at": "2023-02-14T23:04:25.325874",
// 		"store_id": null
// 	},
// 	"error": null
// }

/// Returns corresponding product with id=:prodId
#[utoipa::path(
    get, 
    path = "/product/{id}",
    params(
        ("id", description = "Unique id of products")
    ),
    responses(
        (status = 200, description = "Returns the product with the id", body = QResult<Product>, example = json!(QResult {
            rows: Product {id: 1, name: "product 1".to_owned(), price: BigDecimal::from(10), i18n_name: Some("i18n".to_owned()), i18n_description: Some("description".to_owned()), description: Some("description".to_owned()), created_at: Utc::now().naive_utc(), store_id: Some(1)},
            error: None
        })),
    )
)]
#[get("{prod_id}")]
pub async fn get(app_data: web::Data<AppData>, prod_id: web::Path<i32>) -> HttpResponse {
    match app_data.pg_pool.get() {
        Ok(conn) => product_repo::get_product(conn, prod_id.into_inner()).await,
        _ => HttpResponse::InternalServerError().body("Internal Server Error"),
    }
}

/// Returns a paginated list of products
#[utoipa::path(
    get, 
    path = "/product",
    params(
        PaginationDto,
        OrderBy,
        SearchBy,
        CategoryId,
        StoreId
    ),
    responses(
        (status = 200, description = "Returns a list of products", body = PaginatedResult<Product>, example = json!(PaginatedResult {
            per_page: 10,
            page: 1,
            total_pages: 1,
            result: vec![Product {id: 1, name: "product 1".to_owned(), price: BigDecimal::from(10), i18n_name: Some("i18n".to_owned()), i18n_description: Some("description".to_owned()), description: Some("description".to_owned()), created_at: Utc::now().naive_utc(), store_id: Some(1)}]
        })),
    )
)]
#[get("")]
pub async fn get_many(
    app_data: web::Data<AppData>,
    pagination: Query<PaginationDto>,
    order: Query<OrderBy>,
    search: Query<SearchBy>,
    category_id: Query<CategoryId>,
    store_id: Query<StoreId>,
) -> HttpResponse {
    match app_data.pg_pool.get() {
        Ok(conn) => {
            product_repo::get_many(
                conn,
                pagination.into_inner(),
                order.into_inner().option(),
                search.into_inner(),
                category_id.into_inner().category_id,
                store_id.into_inner().store_id
            )
            .await
        }
        _ => HttpResponse::InternalServerError().body("Internal Server Error"),
    }
}

/// Creates a new Product
#[utoipa::path(
    post, 
    path = "/product",
    request_body (content = ProductDto, content_type = "application/json", example = json!(ProductDto {  name: "product 1".to_owned(), price: 10.10, i18n_name: Some("i18n".to_owned()), i18n_description: Some("description".to_owned()), description: Some("description".to_owned()), store_id: Some(1), category_id: None })),
    responses(
        (status = 200, body = QResult<Product>, example = json!(QResult {
            rows: Product {id: 1, name: "product 1".to_owned(), price: BigDecimal::from(10), i18n_name: Some("i18n".to_owned()), i18n_description: Some("description".to_owned()), description: Some("description".to_owned()), created_at: Utc::now().naive_utc(), store_id: Some(1)},
            error: None
        })),
    )
)]
#[post("")]
pub async fn post(app_data: web::Data<AppData>, prod: Json<ProductDto>) -> HttpResponse {
    match app_data.pg_pool.get() {
        Ok(conn) => product_repo::add_product(conn, prod.into_inner()).await,
        _ => HttpResponse::InternalServerError().body("Internal Server Error"),
    }
}

/// Edits product with corresponding ID
#[utoipa::path(
    put, 
    path = "/product",
    request_body = UpdateProductDto,
    params (
        ("id", description = "id of product")
    ),
    responses(
        (status = 200, body = QResult<Product>, example = json!(QResult {
            rows: Product {id: 1, name: "product 1".to_owned(), price: BigDecimal::from(10), i18n_name: Some("i18n".to_owned()), i18n_description: Some("description".to_owned()), description: Some("description".to_owned()), created_at: Utc::now().naive_utc(), store_id: Some(1)},
            error: None
        })),
    )
)]
#[put("{id}")]
pub async fn update(
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

/// Deletes product with id
#[utoipa::path(
    delete, 
    path = "/product/{id}",
    params(
        ("id", description = "Unique id of products")
    ),
    responses(
        (status = 200, description = "Returns deleted product", body = QResult<Product>, example = json!(QResult {
            rows: Product {id: 1, name: "product 1".to_owned(), price: BigDecimal::from(10), i18n_name: Some("i18n".to_owned()), i18n_description: Some("description".to_owned()), description: Some("description".to_owned()), created_at: Utc::now().naive_utc(), store_id: Some(1)},
            error: None
        })),
    )
)]
#[delete("{id}")]
pub async fn delete(app_data: web::Data<AppData>, id: web::Path<i32>) -> HttpResponse {
    match app_data.pg_pool.get() {
        Ok(conn) => product_repo::delete_product(conn, id.into_inner()).await,
        _ => HttpResponse::InternalServerError().body("Internal Server Error"),
    }
}


/// Attach category to product
#[utoipa::path(
    put, 
    path = "/product/{prod_id}/category/{cat_id}",
    params(
        ("prod_id", description = "Unique id of products"),
        ("cat_id", description = "Unique id of category"),
    ),
    responses(
        (status = 200, body = QResult<ProductsCategories>, example = json!(QResult {
            rows: ProductsCategories { id: 1, category_id: 3, product_id: 5 },
            error: None
        })),
    )
)]
#[put("{prod_id}/category/{cat_id}")]
pub async fn attach_category(
    app_data: web::Data<AppData>,
    path: web::Path<(i32, i32)>,
) -> HttpResponse {
    println!("inside attach");
    match app_data.pg_pool.get() {
        Ok(conn) => product_repo::attach_category(conn, path.0, path.1).await,
        _ => HttpResponse::InternalServerError().body("Internal Server Error"),
    }
}

/// Attach store to product
#[utoipa::path(
    put, 
    path = "/product/{prod_id}/store/{store_id}",
    params(
        ("prod_id", description = "Unique id of products"),
        ("store_id", description = "Unique id of stores"),
    ),
    responses(
        (status = 200, description = "Returns a list of products", body = QResult<Product>, example = json!(QResult {
            rows: Product {id: 1, name: "product 1".to_owned(), price: BigDecimal::from(10), i18n_name: Some("i18n".to_owned()), i18n_description: Some("description".to_owned()), description: Some("description".to_owned()), created_at: Utc::now().naive_utc(), store_id: Some(1)},
            error: None
        })),
    )
)]
#[put("{prod_id}/store/{store_id}")]
pub async fn attach_store(app_data: web::Data<AppData>, path: web::Path<(i32, i32)>) -> HttpResponse {
    match app_data.pg_pool.get() {
        Ok(conn) => product_repo::attach_store(conn, path.0, path.1).await,
        _ => HttpResponse::InternalServerError().body("Internal Server Error"),
    }
}

/// Dettach category from product
#[utoipa::path(
    delete, 
    path = "/product/{prod_id}/category/{cat_id}",
    params(
        ("prod_id", description = "Unique id of products"),
        ("cat_id", description = "Unique id of category"),
    ),
    responses(
        (status = 200, description = "Returns a list of products", body = QResult<ProductsCategories>, example = json!(QResult {
            rows: ProductsCategories { id: 1, category_id: 3, product_id: 5 },
            error: None
        })),
    )
)]
#[delete("{prod_id}/category/{cat_id}")]
pub async fn dettach_category(
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
