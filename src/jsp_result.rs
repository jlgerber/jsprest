use actix_web::{Error, HttpRequest, HttpResponse, Responder};
//use actix_web::Result as AWResult;
use serde::Serialize;
use std::fmt::Debug;
use crate::JspRestError;

#[derive(Debug, Serialize)]
pub struct JspResult<I: Serialize + Debug> {
    //links
    success: bool, 
    data: I,
}

impl<I: Debug + Serialize> JspResult<I> {
    pub fn new(success:bool, data: I) -> JspResult<I>
    where
        I: Serialize + Debug,
    {
        JspResult { success, data }
    }
}

impl<I: Debug + Serialize> Responder for JspResult<I> {
    type Error = Error;
    type Future = Result<HttpResponse, Error>;

    fn respond_to(self, _req: &HttpRequest) -> Self::Future {
        let body = serde_json::to_string(&self)?;
        if self.success {
            Ok(HttpResponse::Ok()
            .content_type("application/json")
            .body(body))
        } else {
            Err(JspRestError::IncorrectData(_req.path().to_string()).into())
        }
        // Create response and set content type
        
    }
}
