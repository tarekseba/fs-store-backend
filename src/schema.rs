// @generated automatically by Diesel CLI.

diesel::table! {
    categories (id) {
        id -> Int4,
        name -> Varchar,
        created_at -> Timestamp,
    }
}

diesel::table! {
    products (id) {
        id -> Int4,
        name -> Varchar,
        price -> Numeric,
        description -> Nullable<Text>,
        created_at -> Timestamp,
        store_id -> Nullable<Int4>,
    }
}

diesel::table! {
    products_categories (id) {
        id -> Int4,
        product_id -> Int4,
        category_id -> Int4,
    }
}

diesel::table! {
    stores (id) {
        id -> Int4,
        name -> Varchar,
        is_holiday -> Bool,
        created_at -> Timestamp,
    }
}

diesel::table! {
    worktimes (id) {
        id -> Int4,
        day_id -> Int4,
        store_id -> Int4,
        am_open -> Nullable<Varchar>,
        am_close -> Nullable<Varchar>,
        pm_open -> Nullable<Varchar>,
        pm_close -> Nullable<Varchar>,
    }
}

diesel::joinable!(products -> stores (store_id));
diesel::joinable!(products_categories -> categories (category_id));
diesel::joinable!(products_categories -> products (product_id));
diesel::joinable!(worktimes -> stores (store_id));

diesel::allow_tables_to_appear_in_same_query!(
    categories,
    products,
    products_categories,
    stores,
    worktimes,
);
