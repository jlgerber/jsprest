use actix_web::{web, App, HttpServer};
use jsprest::{show_index, shows_index};
use structopt::StructOpt;

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

fn main() -> std::io::Result<()> {
    let opt = Opt::from_args();
    let binding = format!("{}:{}", opt.host, opt.port);
    HttpServer::new(|| {
        App::new()
            .route("/projects", web::get().to(shows_index))
            .route("/projects/{project}", web::get().to(show_index))
    })
    .bind(binding.as_str())?
    .run()?;
    Ok(())
}
