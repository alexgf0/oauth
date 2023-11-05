use actix_web::{middleware::Logger, App, HttpServer};
use actix_web::{get, HttpResponse, Responder, HttpRequest};
use dotenv::dotenv;



#[get("/login")]
async fn google_login() -> impl Responder {
    // get the parameters from .env file to make get petition to google
    let redirect_uri = std::env::var("google_redirect_uri").expect("google_redirect_uri must be defined in the .env file.");
    let client_id = std::env::var("google_client_id").expect("google_client_id must be defined in the .env file.");
    let scope = std::env::var("google_scope").expect("google_scope must be defined in the .env file.");


    let body = format!("<a href=\"https://accounts.google.com/o/oauth2/v2/auth?scope={}&redirect_uri={}&response_type=code&client_id={}\">Google Login</a>",
        scope, redirect_uri, client_id);

    HttpResponse::Ok().body(body)
}

#[get("/google/redirect")]
async fn google_redirect(req_headers: HttpRequest) -> impl Responder {
    let parameters = req_headers.query_string().split("&");
    for parameter in parameters {
        let values = parameter.split("=").collect::<Vec<&str>>();

        if values[0] == "code" {
            println!("Your authorization code is: {}", values[1]);
        }
    }
    HttpResponse::Ok().body(format!("Hi"))
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    
    HttpServer::new(|| {
        App::new()
            .service(google_login)
            .service(google_redirect)
            // Enable the logger.
            .wrap(Logger::default())
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

