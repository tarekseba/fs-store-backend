use crate::schema::stores;
use crate::schema::worktimes;
use chrono::NaiveDateTime;
use diesel::QueryableByName;
use diesel::{AsChangeset, Associations, Identifiable, Insertable, Queryable};
use regex::Regex;
use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationError};

use super::Product;

#[derive(Identifiable, Queryable, Serialize, Deserialize, Debug, Clone, QueryableByName)]
#[diesel(table_name = stores)]
pub struct Store {
    pub id: i32,
    pub name: String,
    pub is_holiday: bool,
    pub created_at: NaiveDateTime,
    pub prod_count: i32,
}

#[derive(Serialize)]
pub struct StoreResult {
    pub id: i32,
    pub is_holiday: bool,
    pub created_at: NaiveDateTime,
    pub name: String,
    pub prod_count: i32,
    pub worktimes: Vec<Worktimes>,
}

#[derive(Serialize)]
pub struct StoreResultWithProducts {
    pub id: i32,
    pub name: String,
    pub is_holiday: bool,
    pub created_at: NaiveDateTime,
    pub prod_count: i32,
    pub worktimes: Vec<Worktimes>,
    pub products: Vec<Product>,
}

#[derive(Deserialize, Validate)]
pub struct CreateStoreDto {
    #[validate(length(min = 2, max = 256))]
    pub name: String,
    pub is_holiday: bool,
    pub worktimes: [CreateWorktimeDto; 7],
}

#[derive(Deserialize, Validate)]
pub struct UpdateStoreDto {
    #[validate(length(min = 2, max = 256))]
    pub name: String,
    pub is_holiday: bool,
    #[validate]
    pub worktimes: Vec<UpdateWorktimeDto>,
}

#[derive(Identifiable, Associations, Deserialize, Queryable, Debug, Serialize)]
#[diesel(table_name = worktimes, belongs_to(Store, foreign_key = store_id))]
pub struct Worktimes {
    pub id: i32,
    pub day_id: i32,
    pub store_id: i32,
    pub am_open: Option<String>,
    pub am_close: Option<String>,
    pub pm_open: Option<String>,
    pub pm_close: Option<String>,
}

#[derive(Serialize, Deserialize, Validate)]
pub struct CreateWorktimeDto {
    // store_id: i32,
    #[validate(range(min = 1, max = 7))]
    pub day_id: i32,
    #[validate(custom = "validate_worktimes")]
    pub am_open: Option<String>,
    #[validate(custom = "validate_worktimes")]
    pub am_close: Option<String>,
    #[validate(custom = "validate_worktimes")]
    pub pm_open: Option<String>,
    #[validate(custom = "validate_worktimes")]
    pub pm_close: Option<String>,
}

#[derive(Deserialize, Validate, AsChangeset, Debug)]
#[diesel(table_name = worktimes)]
pub struct UpdateWorktimeDto {
    #[validate(range(min = 1))]
    pub id: i32,
    #[validate(range(min = 1))]
    pub store_id: Option<i32>,
    #[validate(range(min = 1, max = 7))]
    pub day_id: Option<i32>,
    #[validate(custom = "validate_worktimes")]
    pub am_open: Option<String>,
    #[validate(custom = "validate_worktimes")]
    pub am_close: Option<String>,
    #[validate(custom = "validate_worktimes")]
    pub pm_open: Option<String>,
    #[validate(custom = "validate_worktimes")]
    pub pm_close: Option<String>,
}

#[derive(Serialize, Deserialize, AsChangeset, Insertable, Validate)]
#[diesel(table_name = worktimes)]
pub struct InsertableWorktime {
    pub store_id: i32,
    #[validate(range(min = 1, max = 7))]
    pub day_id: i32,
    #[validate(custom = "validate_worktimes")]
    pub am_open: Option<String>,
    #[validate(custom = "validate_worktimes")]
    pub am_close: Option<String>,
    #[validate(custom = "validate_worktimes")]
    pub pm_open: Option<String>,
    #[validate(custom = "validate_worktimes")]
    pub pm_close: Option<String>,
}

pub trait MyInto<T> {
    fn into(self, store_id: i32) -> T;
}

impl MyInto<InsertableWorktime> for CreateWorktimeDto {
    fn into(self, store_id: i32) -> InsertableWorktime {
        InsertableWorktime {
            store_id,
            day_id: self.day_id,
            am_open: self.am_open,
            am_close: self.am_close,
            pm_open: self.pm_open,
            pm_close: self.pm_close,
        }
    }
}

pub trait TransformTo<E> {
    fn transform_to(self, store_id: i32) -> E;
}

impl TransformTo<Vec<InsertableWorktime>> for CreateStoreDto {
    fn transform_to(self, store_id: i32) -> Vec<InsertableWorktime> {
        let mut result = vec![];
        for worktime in self.worktimes {
            result.push(<CreateWorktimeDto as MyInto<InsertableWorktime>>::into(
                worktime, store_id,
            ))
        }
        result
    }
}

pub fn validate_worktimes(worktime: &str) -> Result<(), ValidationError> {
    let time_regex = Regex::new(r#"^(0[0-9]|1[0-2]):[0-5][0-9]$"#).unwrap();
    match time_regex.is_match(worktime) {
        true => Ok(()),
        false => Err(ValidationError::new("Wrong time format : HH:MM")),
    }
}

impl Into<StoreResult> for (Store, Vec<Worktimes>) {
    fn into(self) -> StoreResult {
        StoreResult {
            id: self.0.id,
            name: self.0.name,
            created_at: self.0.created_at,
            is_holiday: self.0.is_holiday,
            prod_count: self.0.prod_count,
            worktimes: self.1,
        }
    }
}
