use crate::{
    config::Config,
    models::{
        Category, CategoryDto, CategoryId, PaginatedResult, Product, ProductDto, QResult, Store,
        StoreId, UpdateCategoryDto, UpdateProductDto, CreateStoreDto, UpdateStoreDto
    },
    repos::pagination::PaginationDto,
    routes::{
        init_category_routes, init_product_routes, init_store_routes, ManyIdsDto, OrderBy, SearchBy, StoresOrderBy, DateFilter
    },
    utils::{create_conn_pool, server_running, AppData, ValidationErrorJsonPayload},
};
use actix_cors::Cors;
use actix_web::{self, main, web, App, HttpServer};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

mod config;
mod models;
mod repos;
mod routes;
mod schema;
mod utils;

#[main]
async fn main() -> std::io::Result<()> {
    let config: Config = Config::load_config();
    let app_data: AppData = create_conn_pool(&config);
    server_running(&config);

    #[derive(OpenApi)]
    #[openapi(
        paths(
            routes::product_routes::get,
            routes::product_routes::get_many,
            routes::product_routes::post,
            routes::product_routes::delete,
            routes::product_routes::update,
            routes::product_routes::attach_category,
            routes::product_routes::attach_store,
            routes::product_routes::dettach_category,
            routes::category_routes::get,
            routes::category_routes::get_many,
            routes::category_routes::post,
            routes::category_routes::delete,
            routes::category_routes::delete_many,
            routes::store_routes::get,
            routes::store_routes::get_many,
            routes::store_routes::post,
            routes::store_routes::update,
            routes::store_routes::delete,
            routes::store_routes::product_count,
        ),
        components(
            schemas(
                PaginatedResult<Product>,
                QResult<Product>,
                ProductDto,
                UpdateProductDto,
                Category,
                Store,
                CreateStoreDto,
                UpdateStoreDto,
                CategoryDto,
                UpdateCategoryDto,
                ManyIdsDto,
                ValidationErrorJsonPayload,
                PaginationDto,
                OrderBy,
                StoresOrderBy,
                SearchBy,
                CategoryId,
                StoreId,
                DateFilter,
                
            )
        ),
        tags(
            (name = "fs-store", description = "FS-Store API endpoints")
        )
    )]
    struct ApiDoc;

    let openapi = ApiDoc::openapi();

    HttpServer::new(move || {
        let cors = Cors::permissive();
        App::new()
            .wrap(cors)
            .app_data(web::Data::new(app_data.clone()))
            .service(web::scope("/category").configure(init_category_routes))
            .service(web::scope("/product").configure(init_product_routes))
            .service(web::scope("/store").configure(init_store_routes))
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}").url("/api-doc/openapi.json", openapi.clone()),
            )
    })
    .bind((config.get_srv_addr(), *config.get_srv_port()))?
    .run()
    .await
}
