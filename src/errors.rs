use failure::Fail;
use actix_web::{ResponseError, web::HttpResponse};

#[derive(Debug, Fail, PartialEq, Clone)]
pub enum JspRestError {
        #[fail(display = "Incorrect Data: {:?}", _0)]
        IncorrectData(String)
}

impl ResponseError for JspRestError {
    fn error_response(&self) -> HttpResponse<Body> {}
    fn render_response(&self) -> HttpResponse<Body> {}
}