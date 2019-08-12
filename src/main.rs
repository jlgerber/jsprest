
#![feature(proc_macro_hygiene, decl_macro)]
use structopt::StructOpt;
#[macro_use] extern crate rocket;
//use rocket;
use rocket::{get, routes};
use rocket_cors::{AllowedHeaders, AllowedOrigins, Error};
use rocket::http::Method;

mod routes;
mod shows;
pub use routes::JspResponse;


// accepted options from rocket
const PV: &[&'static str] = &["prod","production","staging","dev","development"];

#[derive(Debug, StructOpt)]
#[structopt(name = "jsprest", about = "RESTful Jobsystem Police Service")]
struct Opt {
    /// Activate debug mode
    #[structopt(short = "d", long = "debug")]
    debug: bool,
    // /// Set speed
    // #[structopt(short = "h", long = "host", default_value = "localhost")]
    // host: String,
    // /// Port
    // #[structopt(short = "p", long = "port", default_value = "8080")]
    // port: u32,
    /// Set Environment for Rocket
    #[structopt(short = "e", long = "env", default_value = "dev", raw(possible_values="&PV"))]
    env: String
}


fn main() -> Result<(), Error> {
    let opt = Opt::from_args();
    std::env::set_var("ROCKET_ENV", opt.env);
    let allowed_origins = AllowedOrigins::some_regex(&["http://.*:.*"]);
    
    let cors = rocket_cors::CorsOptions {
        allowed_origins,
        allowed_methods: vec![Method::Get].into_iter().map(From::from).collect(),
        allowed_headers: AllowedHeaders::some(&["Authorization", "Accept"]),
        allow_credentials: true,
        ..Default::default()
    }
    .to_cors()?;

    rocket::ignite()
    .mount("/", routes![routes::projects::projects])
    .mount("/", routes![routes::projects::project])
    .attach(cors)
    .launch();

    Ok(())
}