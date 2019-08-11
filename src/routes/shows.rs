use crate::{get_showlist, ShowList};
use actix_web::Responder;

pub fn index() -> impl Responder {
    let result = get_showlist();
    // HttpResponse::Ok().body(&ShowList::new(result))
    ShowList::new(result)
}
