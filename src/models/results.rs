use actix_web::{error::BlockingError, http::StatusCode, web, HttpResponse, HttpResponseBuilder};
use serde::Serialize;
use utoipa::ToSchema;

#[derive(Serialize, ToSchema)]
pub struct QResult<T>
where
    T: Serialize,
{
    pub rows: T,
    pub error: Option<String>,
}

impl<T: Serialize> QResult<T> {
    pub fn new(rows: T, error: Option<String>) -> QResult<T> {
        QResult { rows, error }
    }
}

#[derive(Serialize, ToSchema)]
pub struct PaginatedResult<T>
where
    T: Serialize,
{
    pub result: T,
    pub total_pages: i64,
    pub page: i64,
    pub per_page: i64,
}

impl<T: Serialize> PaginatedResult<T> {
    pub fn new(data: T, total_pages: i64, page: i64, per_page: i64) -> PaginatedResult<T> {
        PaginatedResult {
            result: data,
            total_pages,
            page,
            per_page,
        }
    }
}

pub enum ResultEnum<T: Serialize> {
    Paginated(Result<Result<(T, i64, i64, i64), diesel::result::Error>, BlockingError>),
    NotPaginated(Result<Result<T, diesel::result::Error>, BlockingError>),
}

pub trait CanRespond<T>
where
    T: Serialize,
{
    fn respond(&self, status: StatusCode) -> HttpResponse;
}

impl<T> CanRespond<T> for ResultEnum<T>
where
    T: Serialize,
{
    fn respond(&self, status: StatusCode) -> HttpResponse {
        match self {
            ResultEnum::Paginated(val) => match val {
                Ok(Ok((data, total_pages, page, per_page))) => HttpResponseBuilder::new(status)
                    .json(web::Json(PaginatedResult::new(
                        data,
                        *total_pages,
                        *page,
                        *per_page,
                    ))),
                Ok(Err(err)) => HttpResponseBuilder::new(StatusCode::BAD_REQUEST)
                    .json(web::Json(QResult::new(0, Some(err.to_string())))),
                Err(err) => HttpResponse::InternalServerError()
                    .json(web::Json(QResult::new(0, Some(err.to_string())))),
            },
            ResultEnum::NotPaginated(val) => match val {
                Ok(Ok(val)) => {
                    HttpResponseBuilder::new(status).json(web::Json(QResult::new(val, None)))
                }
                Ok(Err(err)) => HttpResponseBuilder::new(StatusCode::BAD_REQUEST)
                    .json(web::Json(QResult::new(0, Some(err.to_string())))),
                Err(err) => HttpResponse::InternalServerError()
                    .json(web::Json(QResult::new(0, Some(err.to_string())))),
            },
        }
    }
}
