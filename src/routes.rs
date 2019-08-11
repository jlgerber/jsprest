use crate::shows::get_showlist;
use rocket_contrib::json::Json;
use std::fmt::Debug;
use serde::Serialize;

pub mod projects;
pub use projects::projects;

#[derive(serde::Serialize, Debug)]
pub struct JspResponse<I> {
    data: Vec<I>
}

impl<I> JspResponse<I> where I:Serialize+Debug {
    pub fn new(data: Vec<I>) -> JspResponse<I> where I:Serialize+Debug {
        JspResponse { data }
    }
}
