use actix_web::Result as AWResult;

pub mod errors;
pub use errors::JspRestError;
pub mod routes;
pub use routes::shows::index as shows_index;
pub use routes::show::index as show_index;
pub mod jsp_result;
pub use jsp_result::JspResult;
pub mod showlist;
use showlist::{ShowList, get_showlist};


