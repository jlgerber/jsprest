use actix_web::{Error, HttpRequest, HttpResponse, Responder};
//use actix_web::Result as AWResult;
use crate::jsp_result::JspResult;
use serde::Serialize;
use std::fs::read_dir;

#[derive(Debug, Serialize)]
pub struct ShowList {
    shows: Vec<String>,
}

impl ShowList {
    pub fn new(shows: Vec<String>) -> ShowList {
        ShowList { shows }
    }
}

impl Responder for ShowList {
    type Error = Error;
    type Future = Result<HttpResponse, Error>;

    fn respond_to(self, _req: &HttpRequest) -> Self::Future {
        let body = JspResult::new(self);
        let body = serde_json::to_string(&body)?;

        // Create response and set content type
        Ok(HttpResponse::Ok()
            .content_type("application/json")
            .body(body))
    }
}

pub(crate) fn get_showlist() -> Vec<String> {
    let r: Vec<String> = vec![];
    let result = match read_dir("/dd/shows") {
        Ok(readdir) => readdir
            .into_iter()
            // TODO: error handling for unwraps
            .map(|r| r.unwrap().file_name().to_str().unwrap().to_string())
            .collect::<Vec<String>>(),
        Err(_) => r,
    };
    result
}
