#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

use rocket::http::RawStr;

#[get("/")]
fn index() -> &'static str {
    "Hello world!"
}

#[get("/hello")]
fn hello() -> &'static str { "Hello there, you sexy beast" }

#[get("/dynamic/<name>")]
fn dynamic(name: &RawStr) -> String {
    format!("Hello, {}. I'm dad!", name.as_str())
}

#[get("/v4/<name>/<age>/<height>")]
fn person(name: String, age: u8, height: u8) -> String {
    format!("You're dumb {}", name)
}

#[get("/optional?<name>")]
fn optional(name: Option<String>) -> String {
    name.map(|name| format!("Hi there {}", name)).unwrap_or_else(|| "You're dumb".into())
}

mod v3 {
    #[get("/blah2")]
    pub fn blah() -> &'static str {
        "blah blah"
    }
}

fn main() {
    rocket::ignite()
        .mount("/", routes![index, hello])
        .mount("/v2", routes![hello, v3::blah])
        .mount("/v3", routes![dynamic])
        .launch();
}