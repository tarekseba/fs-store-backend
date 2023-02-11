use crate::{
    models::{CanRespond, Category, CategoryDto, ResultEnum, UpdateCategoryDto},
    repos::pagination::{Paginate, PaginationDto},
    routes::{OrderBy, SearchBy, Stringify},
    schema::categories,
    utils::Connection,
};
use actix_web::{http::StatusCode, web, HttpResponse};
use diesel::{self, dsl::sql, prelude::*, sql_types::Text};

pub async fn get_category(mut conn: Connection, cat_id: i32) -> HttpResponse {
    let result = web::block(move || {
        categories::table
            .find(cat_id)
            .get_result::<Category>(&mut conn)
    })
    .await;
    ResultEnum::NotPaginated(result).respond(StatusCode::OK)
}

pub async fn get_many(
    mut conn: Connection,
    pagination: PaginationDto,
    order: Option<OrderBy>,
    search_by: SearchBy,
) -> HttpResponse {
    let result = web::block(move || {
        categories::table
            .filter(categories::name.ilike(search_by.get_name()))
            .order(sql::<Text>(&order.stringify()))
            .paginate(pagination.page)
            .per_page(pagination.per_page)
            .load_and_count_pages::<Category>(&mut conn)
    })
    .await;
    ResultEnum::Paginated(result).respond(StatusCode::OK)
}

pub async fn add_category(mut conn: Connection, cat: CategoryDto) -> HttpResponse {
    let result = web::block(move || {
        diesel::insert_into(categories::table)
            .values(categories::name.eq(cat.name))
            .get_result::<Category>(&mut conn)
    })
    .await;
    ResultEnum::NotPaginated(result).respond(StatusCode::CREATED)
}

pub async fn update_category(
    mut conn: Connection,
    cat: UpdateCategoryDto,
    cat_id: i32,
) -> HttpResponse {
    let result = web::block(move || {
        diesel::update(categories::table.filter(categories::id.eq(cat_id)))
            .set(categories::name.eq(cat.name))
            .get_result::<Category>(&mut conn)
    })
    .await;
    ResultEnum::NotPaginated(result).respond(StatusCode::OK)
}

pub async fn delete_category(mut conn: Connection, cat_id: i32) -> HttpResponse {
    let result = web::block(move || {
        diesel::delete(categories::table.filter(categories::id.eq(cat_id)))
            .get_result::<Category>(&mut conn)
    })
    .await;
    ResultEnum::NotPaginated(result).respond(StatusCode::OK)
}

pub async fn delete_many(mut conn: Connection, cat_ids: Vec<i32>) -> HttpResponse {
    let result = web::block(move || {
        diesel::delete(categories::table.filter(categories::id.eq_any(cat_ids)))
            .get_result::<Category>(&mut conn)
    })
    .await;
    ResultEnum::NotPaginated(result).respond(StatusCode::OK)
}
