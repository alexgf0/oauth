use actix_web::{middleware::Logger, App, HttpServer};
use actix_web::{get, HttpResponse, Responder, HttpRequest};
use reqwest;
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

async fn get_access_token(authorization_code: &str) -> String {
    let client = reqwest::Client::builder().build().unwrap();
    let client_id = std::env::var("google_client_id").expect("google_client_id must be defined in the .env file.");
    let client_secret = std::env::var("google_client_secret").expect("google_client_secret must be defined in the .env file.");
    let redirect_uri = std::env::var("google_redirect_uri").expect("google_redirect_uri must be defined in the .env file.");


    //send get request to https://www.googleapis.com/oauth2/v4/token with our parameters set in the url
    let request = format!("https://www.googleapis.com/oauth2/v4/token?code={}&client_id={}&client_secret={}&grant_type=authorization_code&redirect_uri={}",
        authorization_code, client_id, client_secret, redirect_uri);

    let request = client
        .request(reqwest::Method::POST, request)
        .header("Content-Length", "0");
    
    let response = request.send().await.unwrap();

    let body = response.text().await.unwrap();
    let body: serde_json::Value = serde_json::from_str(&body).unwrap();

    body["access_token"].to_string()
}

async fn get_user_email(access_token: &str) -> String {
    let client = reqwest::Client::builder().build().unwrap();

    let request = client
        .request(reqwest::Method::GET, "https://www.googleapis.com/oauth2/v1/userinfo?alt=json")
        .header("Content-Type", "application/json")
        .header("Authorization", &format!("Bearer {}", access_token));
    
    let response = request.send().await.unwrap();

    let body = response.text().await.unwrap();
    println!("{}", body);
    let body: serde_json::Value = serde_json::from_str(&body).unwrap();

    body["email"].to_string()
}


#[get("/google/redirect")]
async fn google_redirect(req_headers: HttpRequest) -> impl Responder {
    let mut user_email = String::from("");
    let parameters = req_headers.query_string().split("&");
    for parameter in parameters {
        let values = parameter.split("=").collect::<Vec<&str>>();

        if values[0] == "code" {
            println!("calling access_token with {}", values[1]);
            let access_token = get_access_token(values[1]).await;
            println!("access_token: {access_token}");
            user_email = get_user_email(&access_token).await;
        }
    }
    HttpResponse::Ok().body(format!("Your email is: {}", user_email))
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

