use diesel::{prelude::*, Queryable};
use serde::{Deserialize, Serialize};
use validator::Validate;
use chrono::NaiveDateTime;

use crate::schema::categories;

#[derive(Queryable, Validate, Serialize, Deserialize, Debug)]
#[diesel(table_name = categories)]
pub struct Category {
    pub id: i32,
    #[validate(length(min = 3, max = 10))]
    pub name: String,
    pub created_at: NaiveDateTime,
}

#[derive(Validate, Deserialize, Debug)]
pub struct UpdateCategoryDto {
    #[validate(length(min = 3, max = 256))]
    pub name: String,
}

#[derive(Deserialize, Validate, Debug, Insertable)]
#[diesel(table_name = categories)]
pub struct CategoryDto {
    #[validate(length(min = 3, max = 10))]
    pub name: String,
}
