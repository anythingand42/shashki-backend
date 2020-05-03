use actix_files::NamedFile;
use actix_web::{web, App, HttpServer, Result};
use std::path::PathBuf;
use std::env;

fn get_server_port() -> u16 {
    env::var("PORT").ok().and_then(|p| p.parse().ok()).unwrap_or(8080)
}

async fn spa() -> Result<NamedFile> {
    let path: PathBuf = PathBuf::from("./public/index.html");
    Ok(NamedFile::open(path)?)
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(actix_files::Files::new("/public", "public").index_file("index.html"))
            .route("*", web::get().to(spa))
    })
    .bind(("0.0.0.0", get_server_port()))?
    .run()
    .await
}
