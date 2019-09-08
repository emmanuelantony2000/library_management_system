#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_include_static_resources;
extern crate crypto;
extern crate include_dir;
extern crate rocket_contrib;
extern crate simplebase;

mod book;
mod first;
mod issue;

use include_dir::Dir;
use rocket::request::Form;
use rocket::response::Redirect;
use rocket_include_static_resources::StaticResponse;
use std::path::PathBuf;

#[macro_export]
macro_rules! hash {
    ($value:expr) => {{
        use crypto::digest::Digest;
        use crypto::sha3::Sha3;
        let mut h = Sha3::sha3_512();
        h.input_str(&$value[..]);
        h.result_str()
    }};
}

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

#[derive(FromForm, Debug)]
struct Issue {
    name: String,
    class: String,
    id: String,
    book_code: String,
    button: String,
}

/// Renders the Homepage template.
#[get("/")]
fn homepage() -> StaticResponse {
    static_response!("homepage.html")
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
fn book() -> StaticResponse {
    static_response!("book.html")
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
            )
            .unwrap();
            Redirect::to("/book")
        }
        "back" => Redirect::to("/"),
        _ => panic!("Something is wrong in \"book_form\"..."),
    }
}

/// Renders the Book Issue template.
#[get("/issue")]
fn issue() -> StaticResponse {
    static_response!("issue.html")
}

/// Handles the form of Book Issue.
#[post("/issue_form", data = "<data>")]
fn issue_form(data: Form<Issue>) -> Redirect {
    println!("{:?}", data);
    match &data.button[..] {
        "submit" => {
            issue::issue(
                data.name.clone(),
                data.class.clone(),
                data.id.clone(),
                data.book_code.clone(),
            )
            .unwrap();
            Redirect::to("/issue")
        }
        "back" => Redirect::to("/"),
        _ => panic!("Something is wrong in \"issue_form\"..."),
    }
}

fn main() {
    first::init();
    rocket::ignite()
        .attach(StaticResponse::fairing(|mut resources| {
            // static_resources_initialize!(
            //     resources,
            //     "homepage.html",
            //     "templates/homepage.html",
            //     "book.html",
            //     "templates/book.html",
            //     "issue.html",
            //     "templates/issue.html",
            //     "static/images/icons/favicon.ico",
            //     "static/images/icons/favicon.ico",
            //     "static/vendor/bootstrap/css/bootstrap.min.css",
            //     "static/vendor/bootstrap/css/bootstrap.min.css",
            //     "static/fonts/font-awesome-4.7.0/css/font-awesome.min.css",
            //     "static/fonts/font-awesome-4.7.0/css/font-awesome.min.css",
            //     "static/vendor/animate/animate.css",
            //     "static/vendor/animate/animate.css",
            //     "static/vendor/css-hamburgers/hamburgers.min.css",
            //     "static/vendor/css-hamburgers/hamburgers.min.css",
            //     "static/vendor/animsition/css/animsition.min.css",
            //     "static/vendor/animsition/css/animsition.min.css",
            //     "static/vendor/select2/select2.min.css",
            //     "static/vendor/select2/select2.min.css",
            //     "static/vendor/daterangepicker/daterangepicker.css",
            //     "static/vendor/daterangepicker/daterangepicker.css",
            //     "static/css/util.css",
            //     "static/css/util.css",
            //     "static/css/main.css",
            //     "static/css/main.css",
            //     "static/vendor/jquery/jquery-3.2.1.min.js",
            //     "static/vendor/jquery/jquery-3.2.1.min.js",
            //     "static/vendor/animsition/js/animsition.min.js",
            //     "static/vendor/animsition/js/animsition.min.js",
            //     "static/vendor/bootstrap/js/popper.js",
            //     "static/vendor/bootstrap/js/popper.js",
            //     "static/vendor/bootstrap/js/bootstrap.min.js",
            //     "static/vendor/bootstrap/js/bootstrap.min.js",
            //     "static/vendor/select2/select2.min.js",
            //     "static/vendor/select2/select2.min.js",
            //     "static/vendor/daterangepicker/moment.min.js",
            //     "static/vendor/daterangepicker/moment.min.js",
            //     "static/vendor/daterangepicker/daterangepicker.js",
            //     "static/vendor/daterangepicker/daterangepicker.js",
            //     "static/vendor/countdowntime/countdowntime.js",
            //     "static/vendor/countdowntime/countdowntime.js",
            //     "static/js/main.js",
            //     "static/js/main.js"
            // );
            use std::fs;
            fn resource(resources: &mut rocket_include_static_resources::StaticResources, s: &str) {
                let paths = fs::read_dir(s).unwrap();
                for path in paths {
                    let path = path.unwrap().path();
                    let path_str = Box::leak(path.to_str().unwrap().to_string().into_boxed_str());
                    if path.is_dir() {
                        resource(resources, path_str);
                    } else {
                        // resources
                        //     .register_resource_file(path_str, path_str)
                        //     .unwrap();
                        resources.register_resource_static(
                            path_str,
                            concat!(env!("CARGO_MANIFEST_DIR"), "/", path_str),
                            include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/", path_str)),
                        );
                    }
                }
            };
            resource(&mut resources, "./static/");
        }))
        .mount("/", routes![file])
        .mount(
            "/",
            routes![homepage, homepage_form, book, book_form, issue, issue_form],
        )
        .launch();
}

#[get("/<file..>")]
fn file(file: PathBuf) -> StaticResponse {
    println!("{:?}", file);
    static_response!(Box::leak(
        file.to_str().unwrap().to_string().into_boxed_str()
    ))
}
