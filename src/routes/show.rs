use crate::get_showlist;
//use crate::AWResult;
use crate::jsp_result::JspResult;
use actix_web::web;
use actix_web::{Responder, HttpResponse};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Info {
    project: String,
}

pub fn index(info: web::Path<Info>) -> impl Responder {
    let show_list = get_showlist();
    if show_list.contains(&info.project) {
        //return Ok(format!("/dd/shows/{}", info.show));
        return JspResult::new(format!("/dd/shows/{}", info.project));
        //return Ok(body)
    }
    HttpResponse::Err().body("Undef")
    //JspResult::new("UNDEF".to_string())
}
