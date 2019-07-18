#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
extern crate rocket_contrib;

mod book;
mod first;
mod issue;
mod templates;

use rocket::request::Form;
use rocket::response::content;
use rocket::response::Redirect;
use rocket_contrib::serve::StaticFiles;

#[derive(FromForm)]
struct Homepage {
    button: String,
}

#[derive(FromForm)]
struct Book {
    name: String,
    author: String,
    book_code: String,
    button: String,
}

#[derive(FromForm)]
struct Issue {
    name: String,
    class: String,
    id: String,
    book_code: String,
    button: String,
}

/// Renders the Homepage template.
#[get("/")]
fn homepage() -> content::Html<String> {
    content::Html(templates::homepage())
}

/// Handles the button input of the Homepage.
#[post("/homepage_form", data = "<data>")]
fn homepage_form(data: Form<Homepage>) -> Redirect {
    match &data.button[..] {
        "first" => Redirect::to("/book"),
        "second" => Redirect::to("/issue"),
        _ => panic!("Something is wrong in \"homepage_form\"..."),
    }
}

/// Renders the Input Books template.
#[get("/book")]
fn book() -> content::Html<String> {
    content::Html(templates::book())
}

/// Handles the form of the Input Books.
#[post("/book_form", data = "<data>")]
fn book_form(data: Form<Book>) -> Redirect {
    match &data.button[..] {
        "submit" => {
            book::input(
                data.name.clone(),
                data.author.clone(),
                data.book_code.clone(),
            );
            Redirect::to("/book")
        }
        "back" => Redirect::to("/"),
        _ => panic!("Something is wrong in \"homepage_form\"..."),
    }
}

/// Renders the Book Issue template.
#[get("/issue")]
fn issue() -> content::Html<String> {
    content::Html(templates::issue())
}

/// Handles the form of Book Issue.
#[post("/issue_form", data = "<data>")]
fn issue_form(data: Form<Issue>) -> Redirect {
    match &data.button[..] {
        "submit" => {
            issue::issue(
                data.name.clone(),
                data.class.clone(),
                data.id.clone(),
                data.book_code.clone(),
            );
            Redirect::to("/issue")
        }
        "back" => Redirect::to("/"),
        _ => panic!("Something is wrong in \"homepage_form\"..."),
    }
}

fn main() {
    first::init();
    rocket::ignite()
        .mount(
            "/",
            routes![homepage, homepage_form, book, book_form, issue, issue_form],
        )
        .mount(
            "/",
            StaticFiles::from(concat!(env!("CARGO_MANIFEST_DIR"), "/static")),
        )
        .launch();
}
