use actix_files::NamedFile;
use actix_web::{web, App, HttpServer, Result};
use std::path::PathBuf;

async fn spa() -> Result<NamedFile> {
    let path: PathBuf = PathBuf::from("./frontend-app/index.html");
    Ok(NamedFile::open(path)?)
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("*", web::get().to(spa))
            .service(actix_files::Files::new("/", "frontend-app").index_file("index.html"))
    })
    .bind("127.0.0.1:8088")?
    .run()
    .await
}
