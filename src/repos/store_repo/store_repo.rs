use crate::{
    models::{
        CanRespond, CreateStoreDto, Product, ResultEnum, Store, StoreResult,
        StoreResultWithProducts, TransformTo, UpdateStoreDto, Worktimes,
    },
    repos::pagination::{Paginate, PaginationDto},
    routes::{DateFilter, SearchBy, StoresOrderBy, Stringify},
    schema::{stores, stores::*, worktimes, products},
    utils::Connection,
};
use actix_web::{http::StatusCode, web, HttpResponse};
use diesel::{delete, dsl::sql, prelude::*, sql_types::Text, QueryDsl};
use serde::Serialize;

pub async fn get_store(mut conn: Connection, shop_id: i32) -> HttpResponse {
    let result = web::block(move || {
        let store = stores::table
            .find(shop_id)
            .first::<Store>(&mut conn)
            .unwrap();
        let worktimes = Worktimes::belonging_to(&store).load(&mut conn).unwrap();
        let products: Vec<Product> = Product::belonging_to(&store).load(&mut conn).unwrap();
        Ok(StoreResultWithProducts {
            id: store.id,
            name: store.name,
            is_holiday: store.is_holiday,
            created_at: store.created_at,
            worktimes,
            products,
            prod_count: store.prod_count,
        })
    })
    .await;
    ResultEnum::NotPaginated(result).respond(StatusCode::OK)
}

pub async fn get_many(
    mut conn: Connection,
    pagination: PaginationDto,
    order: Option<StoresOrderBy>,
    search_by: SearchBy,
    date: DateFilter,
) -> HttpResponse {
    let result = web::block(move || {
        let results = stores::table
            .filter(
                stores::name
                    .ilike(search_by.get_name())
                    .and(stores::is_holiday.eq_any(vec![
                        search_by.get_is_holiday(),
                        search_by.get_is_holiday_neg(),
                    ]))
                    .and(stores::created_at.between(date.get_after(), date.get_before())),
            )
            .order(sql::<Text>(&order.stringify()))
            .paginate(pagination.page)
            .per_page(pagination.per_page)
            .load_and_count_pages::<Store>(&mut conn);
        let data = results.unwrap();
        let worktimes: Vec<Vec<Worktimes>> = Worktimes::belonging_to(&data.0)
            .load::<Worktimes>(&mut conn)
            .unwrap()
            .grouped_by(&data.0);
        // let id_indices: HashMap<_, _> = data.0
        //     .iter()
        //     .enumerate()
        //     .map(|(i, u)| (u.id, i))
        //     .collect();
        // let mut result = data.0.iter().map(|_| Vec::new()).collect::<Vec<_>>();
        // for child in worktimes {
        //     result[id_indices[&child.store_id]].push(child);
        // }
        Ok((
            // data transformation
            data.0
                .into_iter()
                .zip(worktimes)
                .map(|data: (Store, Vec<Worktimes>)| data.into())
                .collect::<Vec<StoreResult>>(),
            data.1,
            data.2,
            data.3,
        ))
    })
    .await;
    ResultEnum::Paginated(result).respond(StatusCode::OK)
}

pub async fn create_store(mut conn: Connection, store: CreateStoreDto) -> HttpResponse {
    let result = web::block(move || {
        let insert_store = diesel::insert_into(stores::table)
            .values((
                stores::columns::name.eq(&store.name),
                stores::columns::is_holiday.eq(store.is_holiday),
            ))
            .get_result::<Store>(&mut conn);
        match insert_store {
            Ok(insert_store) => {
                let worktimes = store.transform_to(insert_store.id);
                diesel::insert_into(worktimes::table)
                    .values(&worktimes)
                    .execute(&mut conn)
            }
            Err(err) => Err(err),
        }
    })
    .await;
    ResultEnum::NotPaginated(result).respond(StatusCode::OK)
}

pub async fn update_store(
    mut conn: Connection,
    store_id: i32,
    store: UpdateStoreDto,
) -> HttpResponse {
    let result = web::block(move || {
        match diesel::update(stores::table)
            .filter(id.eq(store_id))
            .set((name.eq(&store.name), is_holiday.eq(&store.is_holiday)))
            .get_result::<Store>(&mut conn)
        {
            Ok(val) => {
                // NOTE: Batch update not yet supported by diesel hence the loop
                for worktime in store.worktimes {
                    match diesel::update(worktimes::table)
                        .filter(worktimes::columns::id.eq(worktime.id))
                        .set(&worktime)
                        .get_result::<Worktimes>(&mut conn)
                    {
                        Ok(_) => (),
                        Err(err) => return Err(err),
                    }
                }
                Ok(val)
            }
            Err(err) => Err(err),
        }
    })
    .await;
    ResultEnum::NotPaginated(result).respond(StatusCode::OK)
}
pub async fn delete_store(mut conn: Connection, shop_id: i32) -> HttpResponse {
    let result = web::block(move || {
        delete(stores::table.filter(stores::id.eq(shop_id))).get_result::<Store>(&mut conn)
    })
    .await;
    ResultEnum::NotPaginated(result).respond(StatusCode::OK)
}

pub async fn product_count(mut conn: Connection, store_id: i32) -> HttpResponse {
    let result = web::block(move || {
        let count = products::table
            .filter(products::store_id.eq(store_id))
            .count()
            .get_result(&mut conn);
        Ok(Count {
            count: count.unwrap()
        })
    }).await;
    ResultEnum::NotPaginated(result).respond(StatusCode::OK)
}

#[derive(Serialize)]
struct Count {
    count: i64,
}
