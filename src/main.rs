use chrono::Utc;
use log::*;
use serde_json::json;
use std::path::Path;
use std::{fs, io, path::PathBuf, thread, time};

use actix_web::{get, http::StatusCode, middleware, App, HttpResponse, HttpServer};
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

#[get("/api/dates")]
async fn days_with_photos() -> HttpResponse {
    let response = json!(dirs_with_images(&Opts::parse().image_dir).unwrap());
    HttpResponse::build(StatusCode::OK)
        .content_type("text/json; charset=utf-8")
        .body(response)
}

fn capture_image(image_dir: &Path) {
    let image_dir = image_dir.join(&format!("{}", Utc::today()));
    fs::create_dir_all(&image_dir).unwrap();

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
            Some(code) if !status.success() => {
                error!("raspistill exited with exit status {}", code)
            }
            None => error!("raspistill terminated by signal"),
            _ => info!("captured image {}", image_name),
        },
        Err(err) => error!("{}", err),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "info,actix_server=info,actix_web=info");
    }
    env_logger::init();
    let opts: Opts = Opts::parse();

    info!("Starting camera loop");
    let image_dir = opts.image_dir.clone();
    thread::spawn(move || loop {
        capture_image(&image_dir);
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
            .service(days_with_photos)
    })
    .bind(opts.address)?
    .workers(1)
    .run()
    .await
}

fn dirs_with_images(image_dir: &Path) -> io::Result<Vec<String>> {
    fs::read_dir(image_dir)?
        .filter(|entry| match entry {
            Ok(entry) => entry.path().is_dir(),
            Err(_) => true,
        })
        .map(|entry| Ok(entry?.file_name().into_string().unwrap()))
        .collect()
}
