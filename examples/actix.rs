use actix_web::{App, HttpServer, get};
use mup::{Markup, markup};

#[get("/")]
async fn index() -> Markup {
    markup! {
        @Markup::doctype()
        html lang="en" {
            head { title { "mup + actix-web" } }
            body {
                h1 { "Hello from mup + actix-web" }
                p { "Return " code { "Markup" } " directly from any handler." }
            }
        }
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("listening on http://localhost:3000");
    HttpServer::new(|| App::new().service(index))
        .bind("0.0.0.0:3000")?
        .run()
        .await
}
