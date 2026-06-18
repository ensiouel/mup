#[macro_use]
extern crate rocket;

use mup::{Markup, markup};

#[get("/")]
fn index() -> Markup {
    markup! {
        @Markup::doctype()
        html lang="en" {
            head { title { "mup + rocket" } }
            body {
                h1 { "Hello from mup + rocket" }
                p { "Return " code { "Markup" } " directly from any handler." }
            }
        }
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}
