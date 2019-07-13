#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
extern crate rocket_contrib;

mod book_issue;
mod first;
mod input_books;

use rocket::request::Form;
use rocket_contrib::serve::StaticFiles;
use rocket_contrib::templates::Template;
use std::collections::HashMap;

// use std::io::{self, Read};

// use rocket::data::{FromData, Outcome, Transform, Transformed};
// use rocket::http::Status;
// use rocket::{Data, Outcome::*, Request};

// const NAME_LIMIT: u64 = 256;

#[derive(FromForm)]
struct Button {
    button: String,
}

// enum NameError {
//     Io(io::Error),
//     Parse,
// }

// impl FromData<'_> for Button {
//     type Error = NameError;
//     type Owned = String;
//     type Borrowed = str;

//     fn transform(_: &Request, data: Data) -> Transform<Outcome<Self::Owned, Self::Error>> {
//         let mut stream = data.open().take(NAME_LIMIT);
//         let mut string = String::with_capacity((NAME_LIMIT / 2) as usize);
//         let outcome = match stream.read_to_string(&mut string) {
//             Ok(_) => Success(string),
//             Err(e) => Failure((Status::InternalServerError, NameError::Io(e))),
//         };

//         // Returning `Borrowed` here means we get `Borrowed` in `from_data`.
//         Transform::Borrowed(outcome)
//     }

//     fn from_data(_: &Request, outcome: Transformed<Self>) -> Outcome<Self, Self::Error> {
//         // Retrieve a borrow to the now transformed `String` (an &str). This
//         // is only correct because we know we _always_ return a `Borrowed` from
//         // `transform` above.
//         let s = outcome.borrowed()?;

//         // Return successfully.
//         Success(Button {
//             value: s.to_string(),
//         })
//     }
// }

#[get("/")]
fn homepage() -> Template {
    Template::render("homepage", HashMap::<String, u32>::new())
}

#[post("/homepage_form", data = "<button>")]
fn homepage_form(button: Form<Button>) -> String {
    println!("{:?}", button.into_inner().button);
    // Template::render("homepage", HashMap::<String, u32>::new())
    String::from("Working...")
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
        .mount("/", routes![homepage, homepage_form])
        .mount(
            "/",
            StaticFiles::from(concat!(env!("CARGO_MANIFEST_DIR"), "/static")),
        )
        .attach(Template::fairing())
        .launch();
}
