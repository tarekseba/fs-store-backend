use diesel::{prelude::*, Queryable};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;
use chrono::NaiveDateTime;

use crate::schema::categories;

#[derive(Queryable, Validate, Serialize, Deserialize, Debug, ToSchema)]
#[diesel(table_name = categories)]
pub struct Category {
    pub id: i32,
    #[validate(length(min = 3, max = 50))]
    pub name: String,
    pub created_at: NaiveDateTime,
}

#[derive(Validate, Deserialize, Debug, ToSchema)]
pub struct UpdateCategoryDto {
    #[validate(length(min = 3, max = 256))]
    #[schema(example = "New name")]
    pub name: String,
}

#[derive(Deserialize, Validate, Debug, Insertable, ToSchema)]
#[diesel(table_name = categories)]
pub struct CategoryDto {
    #[validate(length(min = 3, max = 10))]
    #[schema(example = "A category")]
    pub name: String,
}
