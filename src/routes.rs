use std::fmt::Debug;
use serde::Serialize;

pub mod projects;
pub use projects::projects;

/// Response object which shapes the RESTful response
#[derive(serde::Serialize, Debug)]
pub struct JspResponse<I, E> {
    error: Option<E>,
    data: Option<I>,
}

impl<I,E> JspResponse<I,E> where I:Serialize+Debug, E:Serialize {
    pub fn new(data: I) -> JspResponse<I,E> where I:Serialize+Debug {
        JspResponse { error:None, data: Some(data) }
    }
    pub fn error(error:E) -> JspResponse<I,E> where E:Serialize+Debug {
        JspResponse {error:Some(error), data: None }
    }
}
