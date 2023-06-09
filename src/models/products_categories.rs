use crate::models::category::Category;
use crate::models::products::Product;
use crate::schema::products_categories;
use diesel::{Associations, Identifiable, Queryable, QueryableByName};
use serde::Serialize;

#[derive(Identifiable, Queryable, Associations, Debug, Serialize, Clone, QueryableByName)]
#[diesel(belongs_to(Product, foreign_key = product_id))]
#[diesel(belongs_to(Category, foreign_key = category_id))]
#[diesel(table_name = products_categories)]
pub struct ProductsCategories {
    pub id: i32,
    pub product_id: i32,
    pub category_id: i32,
}
