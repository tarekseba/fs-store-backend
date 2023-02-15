use diesel::{
    pg::Pg, prelude::*, query_builder::*, query_dsl::methods::LoadQuery, sql_types::BigInt,
};
use serde::Deserialize;
use utoipa::{IntoParams, ToSchema};
use validator::Validate;

pub const DEFAULT_PER_PAGE: i64 = 20;
pub const DEFAULT_PAGE: i64 = 1;

pub trait Paginate: Sized {
    fn paginate(self, page: Option<i64>) -> Paginated<Self>;
}

impl<T> Paginate for T {
    fn paginate(self, page: Option<i64>) -> Paginated<Self> {
        let page = match page {
            Some(val) => val,
            _ => DEFAULT_PAGE,
        };
        Paginated {
            query: self,
            per_page: DEFAULT_PER_PAGE,
            page,
            offset: (page - 1) * DEFAULT_PER_PAGE,
        }
    }
}

#[derive(Debug, Clone, Copy, QueryId)]
pub struct Paginated<T> {
    query: T,
    page: i64,
    per_page: i64,
    offset: i64,
}

impl<T> Paginated<T> {
    pub fn per_page(self, per_page: Option<i64>) -> Self {
        let per_page = match per_page {
            Some(val) => val,
            _ => DEFAULT_PER_PAGE,
        };
        Paginated {
            per_page,
            offset: (self.page - 1) * per_page,
            ..self
        }
    }

    pub fn load_and_count_pages<'a, U>(
        self,
        conn: &mut PgConnection,
    ) -> QueryResult<(Vec<U>, i64, i64, i64)>
    where
        Self: LoadQuery<'a, PgConnection, (U, i64)>,
    {
        let per_page = self.per_page;
        let page = self.page;
        let results = self.load::<(U, i64)>(conn)?;
        let total = results.get(0).map(|x| x.1).unwrap_or(0);
        let records = results.into_iter().map(|x| x.0).collect();
        let total_pages = (total as f64 / per_page as f64).ceil() as i64;
        Ok((records, total_pages, page, per_page))
    }
}

impl<T: Query> Query for Paginated<T> {
    type SqlType = (T::SqlType, BigInt);
}

impl<T> RunQueryDsl<PgConnection> for Paginated<T> {}

impl<T> QueryFragment<Pg> for Paginated<T>
where
    T: QueryFragment<Pg>,
{
    fn walk_ast<'b>(&'b self, mut out: AstPass<'_, 'b, Pg>) -> QueryResult<()> {
        out.push_sql("SELECT *, COUNT(*) OVER () FROM (");
        self.query.walk_ast(out.reborrow())?;
        out.push_sql(") t LIMIT ");
        out.push_bind_param::<BigInt, _>(&self.per_page)?;
        out.push_sql(" OFFSET ");
        out.push_bind_param::<BigInt, _>(&self.offset)?;
        Ok(())
    }
}

#[derive(Deserialize, Validate, Debug, IntoParams, ToSchema)]
pub struct PaginationDto {
    #[validate(range(min = 1))]
    #[schema(example = 10)]
    pub per_page: Option<i64>,
    #[validate(range(min = 1))]
    #[schema(example = 2)]
    pub page: Option<i64>,
}

impl PaginationDto {
    pub fn get_per_page(&self) -> i32 {
        if let Some(per_page) = self.per_page {
            return per_page as i32;
        } else {
            return DEFAULT_PER_PAGE as i32;
        }
    }
    pub fn get_page(&self) -> i32 {
        if let Some(page) = self.page {
            return page as  i32;
        } else {
            return DEFAULT_PAGE as i32;
        }
    }
}
