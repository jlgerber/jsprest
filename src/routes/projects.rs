use crate::JspResponse;
use crate::shows::get_showlist;
use rocket_contrib::json::Json;


#[get("/projects")]
pub fn projects() -> Json<JspResponse<String>> {
   Json(JspResponse::new(get_showlist()))
}