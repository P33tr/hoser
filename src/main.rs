//https://turreta.com/blog/2020/06/30/send-http-responses-from-actix-web/

use actix_web::{get, App, HttpServer, HttpResponse, Responder, middleware::Logger};
use actix_files as fs;
use std::path::Path;
use std::env;

//#[get("/")]
//async fn pdf() -> Result<NamedFile> {
//    let current_dir = env::current_dir().unwrap();
//    let path = current_dir.join("views").join("index.html");
//    Ok(NamedFile::open(path)?)
//}

#[get("/example")]
async fn example() -> impl Responder{
    println!("in example ....");
    HttpResponse::Ok()
        .content_type("text/html")
        .body("<p>Hello peter</p>")

}

#[actix_web::main]
async fn main() -> std::io::Result<()>{

    println!("hoser running ....");
    let current_dir = env::current_dir().unwrap();
    let path = current_dir.join("src").join("views");
    println!("Serving files from: {:?}",path);
    HttpServer::new(move|| {
        App::new()
            .wrap(Logger::default())
            .service(
            fs::Files::new("/views",path.to_str().unwrap() )
                .show_files_listing()
                .index_file("index.html")
                .use_last_modified(true),
        )
            .service(example)
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await

}
