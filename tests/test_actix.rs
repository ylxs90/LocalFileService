use std::path::PathBuf;
use actix_web::{web, App, HttpServer, Result};
use actix_files::Files;

#[actix_web::test]
async fn test() -> std::io::Result<()> {
    let files_path = PathBuf::from("/home/hxiao");

    HttpServer::new(move || {
        App::new()
            .service(Files::new("/files", &files_path).show_files_listing())
            .route("/", web::get().to(index))
    })
        .bind("0.0.0.0:8080")?
        .run()
        .await
}

async fn index() -> Result<actix_files::NamedFile> {
    Ok(actix_files::NamedFile::open("static/index.html")?)
}