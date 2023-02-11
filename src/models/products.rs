use bigdecimal::{BigDecimal, FromPrimitive};
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::{models::Store, schema::products};

use super::{Category, ProductsCategories};

#[derive(Identifiable, Queryable, Validate, Associations, Serialize, Deserialize, Debug)]
#[diesel(table_name = products, belongs_to(Store))]
pub struct Product {
    pub id: i32,
    pub name: String,
    pub i18n_name: Option<String>,
    pub price: BigDecimal,
    pub description: Option<String>,
    pub i18n_description: Option<String>,
    pub created_at: NaiveDateTime,
    pub store_id: Option<i32>,
}

#[derive(Deserialize, Validate)]
pub struct ProductDto {
    #[validate(length(min = 3, max = 256))]
    pub name: String,
    #[validate(length(min = 3, max = 256))]
    pub i18n_name: Option<String>,
    #[validate(range(min = 0, max = 1000000))]
    pub price: f64,
    #[validate(length(min = 3, max = 1000))]
    pub description: Option<String>,
    #[validate(length(min = 3, max = 1000))]
    pub i18n_description: Option<String>,
    #[validate(range(min = 1))]
    pub category_id: Option<u64>,
    #[validate(range(min = 1))]
    pub store_id: Option<i32>,
}

impl Into<InsertableProduct> for ProductDto {
    fn into(self) -> InsertableProduct {
        InsertableProduct {
            name: self.name,
            i18n_name: self.i18n_name,
            description: self.description,
            i18n_description: self.i18n_description,
            price: BigDecimal::from_f64(self.price).expect("Float conversion error"),
            store_id: self.store_id,
        }
    }
}

#[derive(Deserialize, Validate, Debug)]
pub struct UpdateProductDto {
    #[validate(range(min = 1))]
    pub id: Option<i32>,
    #[validate(length(min = 3, max = 256))]
    pub name: String,
    #[validate(length(min = 3, max = 256))]
    pub i18n_name: Option<String>,
    #[validate(range(min = 1, max = 1000000))]
    pub price: f64,
    #[validate(length(min = 3, max = 1000))]
    pub description: Option<String>,
    #[validate(length(min = 3, max = 1000))]
    pub i18n_description: Option<String>,
    #[validate(range(min = 1))]
    pub store_id: Option<i32>,
}

impl Into<InsertableProduct> for UpdateProductDto {
    fn into(self) -> InsertableProduct {
        InsertableProduct {
            name: self.name,
            i18n_name: self.i18n_name,
            description: self.description,
            i18n_description: self.i18n_description,
            price: BigDecimal::from_f64(self.price).expect("Product price conversion error"),
            store_id: self.store_id,
        }
    }
}

#[derive(Insertable, AsChangeset)]
#[diesel(table_name = products)]
pub struct InsertableProduct {
    pub name: String,
    pub i18n_name: Option<String>,
    pub price: BigDecimal,
    pub description: Option<String>,
    pub i18n_description: Option<String>,
    pub store_id: Option<i32>,
}

#[derive(Debug, Serialize)]
pub struct ProductsResult {
    pub id: i32,
    pub name: String,
    pub i18n_name: Option<String>,
    pub price: BigDecimal,
    pub description: Option<String>,
    pub i18n_description: Option<String>,
    pub created_at: NaiveDateTime,
    pub store_id: Option<i32>,
    pub categories: Vec<Category>,
}

impl Into<ProductsResult> for (Product, Vec<(ProductsCategories, Category)>) {
    fn into(self) -> ProductsResult {
        ProductsResult {
            id: self.0.id,
            name: self.0.name,
            i18n_name: self.0.i18n_name,
            price: self.0.price,
            description: self.0.description,
            i18n_description: self.0.i18n_description,
            created_at: self.0.created_at,
            store_id: self.0.store_id,
            categories: self.1.into_iter().map(|tup| tup.1).collect(),
        }
    }
}
