use actix_web::{
    error::InternalError, http::StatusCode, HttpRequest, HttpResponse, HttpResponseBuilder,
};
use actix_web_validator::Error;
use serde::Serialize;
use std::fmt::Display;
use utoipa::ToSchema;

#[derive(Serialize, ToSchema)]
pub struct ValidationErrorJsonPayload {
    #[schema(example = "validation error")]
    pub message: String,
    #[schema(example = json!(vec!["name", "price"]))]
    pub fields: Vec<String>,
}

impl From<&validator::ValidationErrors> for ValidationErrorJsonPayload {
    fn from(error: &validator::ValidationErrors) -> Self {
        ValidationErrorJsonPayload {
            message: "Validation error".to_owned(),
            fields: error
                .field_errors()
                .iter()
                .map(|(field, _)| field.to_string())
                .collect(),
        }
    }
}

pub fn json_error_handler<'r>(errors: Error, _req: &'r HttpRequest) -> actix_web::Error {
    let json_error = match &errors {
        Error::Validate(error) => ValidationErrorJsonPayload::from(error),
        _ => ValidationErrorJsonPayload {
            message: errors.to_string(),
            fields: Vec::new(),
        },
    };
    InternalError::from_response(errors, HttpResponse::BadRequest().json(json_error)).into()
}

#[derive(Debug, Serialize)]
pub struct NotFoundError {
    error: String,
}

impl Display for NotFoundError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.error)
    }
}

impl std::error::Error for NotFoundError {}
impl actix_web::error::ResponseError for NotFoundError {
    fn status_code(&self) -> StatusCode {
        StatusCode::BAD_REQUEST
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponseBuilder::new(StatusCode::BAD_REQUEST).json(self)
    }
}
