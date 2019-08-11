
#![feature(proc_macro_hygiene, decl_macro)]
use structopt::StructOpt;
mod routes;
mod shows;
pub use routes::JspResponse;

#[macro_use] extern crate rocket;

#[derive(Debug, StructOpt)]
#[structopt(name = "jsprest", about = "RESTful Jobsystem Police Service")]
struct Opt {
    /// Activate debug mode
    #[structopt(short = "d", long = "debug")]
    debug: bool,
    /// Set speed
    #[structopt(short = "h", long = "host", default_value = "localhost")]
    host: String,
    /// Port
    #[structopt(short = "p", long = "port", default_value = "8080")]
    port: u32,
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

fn main() {
    rocket::ignite().mount("/", routes![routes::projects::projects]).launch();
}