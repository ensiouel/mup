use axum::{Router, routing::get};
use mup::{Markup, markup};

async fn index() -> Markup {
    markup! {
        @Markup::doctype()
        html lang="en" {
            head { title { "mup + axum" } }
            body {
                h1 { "Hello from mup + axum" }
                p { "Return " code { "Markup" } " directly from any handler." }
            }
        }
    }
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(index));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("listening on http://localhost:3000");
    axum::serve(listener, app).await.unwrap();
}
