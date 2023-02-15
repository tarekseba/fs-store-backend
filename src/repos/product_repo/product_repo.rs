use crate::{
    models::{
        CanRespond, Category, InsertableProduct, Product, ProductDto, ProductsCategories,
        ProductsResult, ResultEnum, Store, UpdateProductDto,
    },
    repos::pagination::PaginationDto,
    routes::{OrderBy, SearchBy, Stringify},
    schema::{categories, products, products_categories},
    utils::Connection,
};
use actix_web::{http::StatusCode, web, HttpResponse};
use diesel::{
    self,
    prelude::*,
    sql_query,
    sql_types::{Integer, Text},
};

pub async fn get_product(mut conn: Connection, prod_id: i32) -> HttpResponse {
    let result = web::block(move || {
        products::table.find(prod_id).first::<Product>(&mut conn)
        //TODO: get categories
    })
    .await;
    ResultEnum::NotPaginated(result).respond(StatusCode::OK)
}

enum Test {
    WithPc(Result<Vec<(Product, Option<Store>, Option<ProductsCategories>)>, diesel::result::Error>),
    WithoutPc(Result<Vec<(Product, Option<Store>)>, diesel::result::Error>)
}

pub async fn get_many(
    mut conn: Connection,
    pagination: PaginationDto,
    order: Option<OrderBy>,
    search: SearchBy,
    category_id: Option<i32>,
    store: Option<i32>
) -> HttpResponse {
    let result = web::block(move || {
        // 1st DB call
        let res = if let Some(cat_id) = category_id {
            let mut db_query_one = String::from("SELECT * from products p left join stores s on s.id = p.store_id");
            db_query_one.push_str(" right join products_categories pc on pc.product_id = p.id and pc.category_id = $1");
            let mut db_query_two = format!(" WHERE (p.name ILIKE $2 OR p.description ILIKE $3) ");
            if let Some(id) = store {
                db_query_two.push_str(&format!("AND p.store_id = {}", id))
            }
            db_query_two.push_str(&format!(" ORDER BY p.{} LIMIT $4 OFFSET $5", order.stringify()));
            db_query_one.push_str(&db_query_two);
            Test::WithPc(sql_query(db_query_one)
                .bind::<Integer, _>(cat_id as i32)
                .bind::<Text, _>(search.get_name())
                .bind::<Text, _>(search.get_description())
                .bind::<Integer, _>(pagination.get_per_page())
                .bind::<Integer, _>((pagination.get_page() - 1) * pagination.get_per_page())
                .load::<(Product, Option<Store>, Option<ProductsCategories>)>(&mut conn))
        } else {
            let mut db_query_one = String::from("SELECT distinct p.id, p.name, p.i18n_name, p.description, p.i18n_description, p.price, p.store_id, p.created_at, s.id, s.created_at, s.is_holiday, s.name, s.prod_count from products p left join stores s on s.id = p.store_id");
            let mut db_query_two = String::from(" left join products_categories pc on pc.product_id = p.id WHERE (p.name ILIKE $1 OR p.description ILIKE  $2) ");
            if let Some(id) = store {
                db_query_two.push_str(&format!("AND p.store_id = {}", id))
            }
            db_query_two.push_str(&format!(" ORDER BY p.{} LIMIT $3 OFFSET $4", order.stringify()));
            db_query_one.push_str(&db_query_two);
            let res = sql_query(db_query_one)
                .bind::<Text,_>(search.get_name())
                .bind::<Text,_>(search.get_description())
                .bind::<Integer,_>(pagination.get_per_page())
                .bind::<Integer,_>((pagination.get_page() - 1) * pagination.get_per_page())
                .load::<(Product, Option<Store>)>(&mut conn);
            Test::WithoutPc(res)
        };
        let (products, _) = match res {
            Test::WithPc(val) => {
                let data = val.unwrap();
                let products = data.clone()
                    .into_iter()
                    .map(|data: (Product, Option<Store>, Option<ProductsCategories>)| data.0)
                    .collect::<Vec<Product>>();
                let products_stores = data
                    .into_iter()
                    .map(|data: (Product, Option<Store>, Option<ProductsCategories>)| data.1)
                    .collect::<Vec<Option<Store>>>();
                (products, products_stores)
            },
            Test::WithoutPc(val) => {
                let data = val.unwrap();
                let products = data.clone()
                    .into_iter()
                    .map(|data: (Product, Option<Store>)| data.0)
                    .collect::<Vec<Product>>();
                let products_stores = data
                    .into_iter()
                    .map(|data: (Product, Option<Store>)| data.1)
                    .collect::<Vec<Option<Store>>>();
                (products, products_stores)
            },
        };
        // 2nd DB call
        let cats = ProductsCategories::belonging_to(&products)
            .inner_join(categories::table)
            .load::<(ProductsCategories, Category)>(&mut conn)
            .unwrap()
            .grouped_by(&products);
        let x = {
            if products.len() >= pagination.get_per_page() as usize {
                pagination.get_page() + 1
            } else {
                pagination.get_page()
            }
        };
        Ok((
            // data transformation
            products
                .into_iter()
                .zip(cats)
                .map(|data: (Product, Vec<(ProductsCategories, Category)>)| data.into())
                .collect::<Vec<ProductsResult>>()
            ,
            x as i64,
            pagination.get_page() as i64,
            pagination.get_per_page() as i64 
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
