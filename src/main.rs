#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
extern crate rocket_contrib;

mod book_issue;
mod first;
mod input_books;

use rocket_contrib::serve::StaticFiles;
use rocket_contrib::templates::Template;
use std::collections::HashMap;

#[get("/")]
fn homepage() -> Template {
    Template::render("homepage", HashMap::<String, u32>::new())
}

fn main() {
    println!("Hello, world!");
    first::init();
    input_books::input(
        "Hello".to_string(),
        "Hello".to_string(),
        "Hello".to_string(),
    );
    book_issue::issue(
        "Hello".to_string(),
        "Hello".to_string(),
        "Hello".to_string(),
        "Hello".to_string(),
    );
    rocket::ignite()
        .mount("/", routes![homepage])
        .mount(
            "/",
            StaticFiles::from(concat!(env!("CARGO_MANIFEST_DIR"), "/static")),
        )
        .attach(Template::fairing())
        .launch();
}
