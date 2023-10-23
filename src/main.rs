use actix_web::{middleware::Logger, App, HttpServer};
use actix_files::Files;
use actix_web::{get, HttpResponse, Responder};

#[get("/test")]
async fn test_endpoint() -> impl Responder {
    HttpResponse::Ok().body("Hi") 
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(test_endpoint)
            //termporary to test the development of the database
            // We allow the visitor to see an index of the images at `/images`.
            //.service(Files::new("/images", "static/images/").show_files_listing())
            // We allow the visitor to see an index of the javascript files at `/js`.
            .service(Files::new("/js", "main/static/js/").show_files_listing())
            .service(Files::new("/imgs", "main/static/imgs").show_files_listing())
            .service(Files::new("/css", "main/static/css").show_files_listing())
            // Enable the logger.
            .wrap(Logger::default())
    })
    //.bind(("178.79.142.51", 80))?
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

