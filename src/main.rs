use chrono::Utc;
use log::*;
use std::{thread, time, path::PathBuf};

use actix_web::{get, middleware, App, HttpResponse, http::StatusCode, HttpServer};
use clap::Clap;

#[derive(Clap, Clone)]
#[clap(version = "1.0")]
struct Opts {
    /// Path to the place to store the timelapse images
    #[clap(short, long, env = "IMAGE_DIR", default_value = "data")]
    image_dir: PathBuf,

    /// The address to listen on
    #[clap(short, long, env = "ADDR", default_value = "127.0.0.1:8181")]
    address: String,
}

#[get("/api")]
async fn api() -> HttpResponse {
    HttpResponse::build(StatusCode::OK)
        .content_type("text/json; charset=utf-8")
        .body("{}")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "info,actix_server=info,actix_web=info");
    }
    env_logger::init();

    let opts: Opts = Opts::parse();

    let image_dir = opts.image_dir.clone();
    info!("Starting camera loop");
    thread::spawn(move || loop {
        let image_name = format!("{}.jpg", Utc::now().to_rfc3339());
        let status = std::process::Command::new("raspistill")
            .args(&[
                "-t",
                "1000",
                "-ss",
                "10000",
                "--awb",
                "greyworld",
                "-o",
                image_dir.join(&image_name).to_str().unwrap(),
            ])
            .status();
        match status {
                Ok(status) => match status.code() {
                    Some(code) if !status.success() => error!("raspistill exited with exit status {}", code),
                    None => error!("raspistill terminated by signal"),
                    _ => info!("captured image {}", image_name),
                },
                Err(err) => error!("{}", err),
            }
        thread::sleep(time::Duration::from_secs(60));
    });

    let image_dir = opts.image_dir.clone();
    info!("Starting webserver on {}", &opts.address);
    HttpServer::new(move || {
        App::new()
            .wrap(middleware::DefaultHeaders::new().header("X-Version", "0.2"))
            .wrap(middleware::Compress::default())
            .wrap(middleware::Logger::default())
            .service(actix_files::Files::new("/images", &image_dir).show_files_listing())
            .service(api)
    })
    .bind(opts.address)?
    .workers(1)
    .run()
    .await
}
