use crate::{
    models::{
        CanRespond, Category, InsertableProduct, Product, ProductDto, ProductsCategories,
        ProductsResult, ResultEnum, Store, UpdateProductDto,
    },
    repos::pagination::{Paginate, PaginationDto},
    routes::{OrderBy, SearchBy, Stringify},
    schema::{categories, products, products_categories, stores},
    utils::Connection,
};
use actix_web::{http::StatusCode, web, HttpResponse};
use diesel::{self, dsl::sql, prelude::*, sql_types::Text};

pub async fn get_product(mut conn: Connection, prod_id: i32) -> HttpResponse {
    let result = web::block(move || {
        products::table.find(prod_id).first::<Product>(&mut conn)
        //TODO: get categories
    })
    .await;
    ResultEnum::NotPaginated(result).respond(StatusCode::OK)
}

pub async fn get_many(
    mut conn: Connection,
    pagination: PaginationDto,
    order: Option<OrderBy>,
    search: SearchBy,
) -> HttpResponse {
    let result = web::block(move || {
        // 1st DB call
        let results = products::table
            .filter(
                products::name
                    .ilike(search.get_name())
                    .or(products::description.ilike(search.get_description())),
            )
            .order(sql::<Text>(&format!("products.{}", &order.stringify().to_owned())))
            .left_join(stores::table)
            .paginate(pagination.page)
            .per_page(pagination.per_page)
            .load_and_count_pages::<(Product, Option<Store>)>(&mut conn);
        let data = results.unwrap();
        let products = data.clone()
            .0
            .into_iter()
            .map(|data: (Product, Option<Store>)| data.0)
            .collect::<Vec<Product>>();
        let products_stores = data
            .0
            .into_iter()
            .map(|data: (Product, Option<Store>)| data.1)
            .collect::<Vec<Option<Store>>>();
        // 2nd DB call
        let cats = ProductsCategories::belonging_to(&products)
            .inner_join(categories::table)
            .load::<(ProductsCategories, Category)>(&mut conn)
            .unwrap()
            .grouped_by(&products);
        println!("{:?}", cats);
        Ok((
            // data transformation
            products
                .into_iter()
                .zip(cats)
                .zip(products_stores)
                .map(|data: ((Product, Vec<(ProductsCategories, Category)>), Option<Store>)| data.into())
                .collect::<Vec<ProductsResult>>()
            ,
            data.1,
            data.2,
            data.3,
        ))
    })
    .await;
    ResultEnum::Paginated(result).respond(StatusCode::OK)
}

pub async fn add_product(mut conn: Connection, prod: ProductDto) -> HttpResponse {
    let result = web::block(move || {
        diesel::insert_into(products::table)
            .values(<ProductDto as Into<InsertableProduct>>::into(prod))
            .get_result::<Product>(&mut conn)
        //TODO: Add product category if Some(prod.category_id)
    })
    .await;
    ResultEnum::NotPaginated(result).respond(StatusCode::CREATED)
}

pub async fn update_product(
    mut conn: Connection,
    prod_id: i32,
    prod: UpdateProductDto,
) -> HttpResponse {
    let result = web::block(move || {
        diesel::update(products::table)
            .filter(products::columns::id.eq(prod_id))
            .set(&<UpdateProductDto as Into<InsertableProduct>>::into(prod))
            .get_result::<Product>(&mut conn)
        //TODO: Add product category if Some(prod.category_id)
        //FIXME: Rather create new endpoint
    })
    .await;
    ResultEnum::NotPaginated(result).respond(StatusCode::CREATED)
}

pub async fn delete_product(mut conn: Connection, prod_id: i32) -> HttpResponse {
    let result = web::block(move || {
        diesel::delete(products::table.filter(products::id.eq(prod_id)))
            .get_result::<Product>(&mut conn)
    })
    .await;
    ResultEnum::NotPaginated(result).respond(StatusCode::OK)
}

pub async fn attach_category(mut conn: Connection, prod_id: i32, cat_id: i32) -> HttpResponse {
    let result = web::block(move || {
        // match products::table
        //     .filter(products::columns::id.eq(prod_id))
        //     .get_result::<Product>(&mut conn)
        // {
        //     Ok(_) => {
        //         match categories::table
        //             .filter(categories::id.eq(cat_id))
        //             .get_result::<Category>(&mut conn)
        //         {
        //             Ok(_) => diesel::insert_into(products_categories::table)
        //                 .values((
        //                     products_categories::columns::product_id.eq(prod_id),
        //                     products_categories::columns::category_id.eq(cat_id),
        //                 ))
        //                 .execute(&mut conn),
        //             Err(err) => Err(err),
        //         }
        //     }
        //     Err(err) => return Err(err),
        diesel::insert_into(products_categories::table)
            .values((
                products_categories::columns::product_id.eq(prod_id),
                products_categories::columns::category_id.eq(cat_id),
            ))
            .get_result::<ProductsCategories>(&mut conn)
        // }
    })
    .await;
    ResultEnum::NotPaginated(result).respond(StatusCode::OK)
}

pub async fn attach_store(mut conn: Connection, prod_id: i32, store_id: i32) -> HttpResponse {
    let result = web::block(move || {
        diesel::update(products::table)
            .filter(products::id.eq(prod_id))
            .set(products::columns::store_id.eq(store_id))
            .get_result::<Product>(&mut conn)
    })
    .await;
    ResultEnum::NotPaginated(result).respond(StatusCode::OK)
}

pub async fn dettach_category(mut conn: Connection, prod_id: i32, cat_id: i32) -> HttpResponse {
    let result = web::block(move || {
        diesel::delete(
            products_categories::table
                .filter(products_categories::columns::product_id.eq(prod_id))
                .filter(products_categories::columns::category_id.eq(cat_id)),
        )
        .get_result::<ProductsCategories>(&mut conn)
        // }
    })
    .await;
    ResultEnum::NotPaginated(result).respond(StatusCode::OK)
}
