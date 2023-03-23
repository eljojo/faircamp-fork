use actix_files::Files;
use actix_web::{App, HttpServer};
use std::path::Path;

#[actix_web::main]
pub async fn serve_preview(build_dir: &Path) {
    let build_dir_moving = build_dir.to_owned();
    println!("Serving the site preview at http://localhost:8080 (open this address in your browser)");

    let server = HttpServer::new(move || {
        App::new()
            .service(
                Files::new("/", build_dir_moving.clone()).index_file("index.html")
            )
    })
    .bind(("127.0.0.1", 8080))
    .unwrap()
    .run();

    let open_browser = || async {
        if webbrowser::open("http://localhost:8080").is_err() {
            error!("Could not open browser for previewing the site")
        }
    };

    tokio::join!(server, open_browser()).0.unwrap();
}