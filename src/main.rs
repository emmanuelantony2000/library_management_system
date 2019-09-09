#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_include_static_resources;
extern crate crypto;
extern crate rocket_contrib;
extern crate simplebase;
extern crate webbrowser;

mod book;
mod first;
mod issue;

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
    if webbrowser::open("http://localhost:8000").is_err() {
        panic!("Web browser could not be opened...");
    }

    first::init();
    rocket::ignite()
        .attach(StaticResponse::fairing(|resources| {
            static_resources_initialize!(
                resources,
                "homepage.html",
                "templates/homepage.html",
                "book.html",
                "templates/book.html",
                "issue.html",
                "templates/issue.html",
                "static/css/main.css",
                "static/css/main.css",
                "static/css/util.css",
                "static/css/util.css",
                "static/images/icons/favicon.ico",
                "static/images/icons/favicon.ico",
                "static/js/main.js",
                "static/js/main.js",
                "static/fonts/poppins/Poppins-ExtraLight.ttf",
                "static/fonts/poppins/Poppins-ExtraLight.ttf",
                "static/fonts/poppins/Poppins-ThinItalic.ttf",
                "static/fonts/poppins/Poppins-ThinItalic.ttf",
                "static/fonts/poppins/Poppins-ExtraLightItalic.ttf",
                "static/fonts/poppins/Poppins-ExtraLightItalic.ttf",
                "static/fonts/poppins/Poppins-BoldItalic.ttf",
                "static/fonts/poppins/Poppins-BoldItalic.ttf",
                "static/fonts/poppins/Poppins-Light.ttf",
                "static/fonts/poppins/Poppins-Light.ttf",
                "static/fonts/poppins/Poppins-Medium.ttf",
                "static/fonts/poppins/Poppins-Medium.ttf",
                "static/fonts/poppins/Poppins-SemiBoldItalic.ttf",
                "static/fonts/poppins/Poppins-SemiBoldItalic.ttf",
                "static/fonts/poppins/Poppins-ExtraBoldItalic.ttf",
                "static/fonts/poppins/Poppins-ExtraBoldItalic.ttf",
                "static/fonts/poppins/Poppins-ExtraBold.ttf",
                "static/fonts/poppins/Poppins-ExtraBold.ttf",
                "static/fonts/poppins/Poppins-BlackItalic.ttf",
                "static/fonts/poppins/Poppins-BlackItalic.ttf",
                "static/fonts/poppins/Poppins-Regular.ttf",
                "static/fonts/poppins/Poppins-Regular.ttf",
                "static/fonts/poppins/Poppins-LightItalic.ttf",
                "static/fonts/poppins/Poppins-LightItalic.ttf",
                "static/fonts/poppins/Poppins-Bold.ttf",
                "static/fonts/poppins/Poppins-Bold.ttf",
                "static/fonts/poppins/Poppins-Black.ttf",
                "static/fonts/poppins/Poppins-Black.ttf",
                "static/fonts/poppins/Poppins-Thin.ttf",
                "static/fonts/poppins/Poppins-Thin.ttf",
                "static/fonts/poppins/Poppins-SemiBold.ttf",
                "static/fonts/poppins/Poppins-SemiBold.ttf",
                "static/fonts/poppins/Poppins-Italic.ttf",
                "static/fonts/poppins/Poppins-Italic.ttf",
                "static/fonts/poppins/Poppins-MediumItalic.ttf",
                "static/fonts/poppins/Poppins-MediumItalic.ttf",
                "static/fonts/font-awesome-4.7.0/css/font-awesome.css",
                "static/fonts/font-awesome-4.7.0/css/font-awesome.css",
                "static/fonts/font-awesome-4.7.0/css/font-awesome.min.css",
                "static/fonts/font-awesome-4.7.0/css/font-awesome.min.css",
                "static/fonts/font-awesome-4.7.0/less/list.less",
                "static/fonts/font-awesome-4.7.0/less/list.less",
                "static/fonts/font-awesome-4.7.0/less/stacked.less",
                "static/fonts/font-awesome-4.7.0/less/stacked.less",
                "static/fonts/font-awesome-4.7.0/less/core.less",
                "static/fonts/font-awesome-4.7.0/less/core.less",
                "static/fonts/font-awesome-4.7.0/less/fixed-width.less",
                "static/fonts/font-awesome-4.7.0/less/fixed-width.less",
                "static/fonts/font-awesome-4.7.0/less/variables.less",
                "static/fonts/font-awesome-4.7.0/less/variables.less",
                "static/fonts/font-awesome-4.7.0/less/rotated-flipped.less",
                "static/fonts/font-awesome-4.7.0/less/rotated-flipped.less",
                "static/fonts/font-awesome-4.7.0/less/font-awesome.less",
                "static/fonts/font-awesome-4.7.0/less/font-awesome.less",
                "static/fonts/font-awesome-4.7.0/less/icons.less",
                "static/fonts/font-awesome-4.7.0/less/icons.less",
                "static/fonts/font-awesome-4.7.0/less/path.less",
                "static/fonts/font-awesome-4.7.0/less/path.less",
                "static/fonts/font-awesome-4.7.0/less/animated.less",
                "static/fonts/font-awesome-4.7.0/less/animated.less",
                "static/fonts/font-awesome-4.7.0/less/bordered-pulled.less",
                "static/fonts/font-awesome-4.7.0/less/bordered-pulled.less",
                "static/fonts/font-awesome-4.7.0/less/larger.less",
                "static/fonts/font-awesome-4.7.0/less/larger.less",
                "static/fonts/font-awesome-4.7.0/less/mixins.less",
                "static/fonts/font-awesome-4.7.0/less/mixins.less",
                "static/fonts/font-awesome-4.7.0/less/screen-reader.less",
                "static/fonts/font-awesome-4.7.0/less/screen-reader.less",
                "static/fonts/font-awesome-4.7.0/scss/_stacked.scss",
                "static/fonts/font-awesome-4.7.0/scss/_stacked.scss",
                "static/fonts/font-awesome-4.7.0/scss/_variables.scss",
                "static/fonts/font-awesome-4.7.0/scss/_variables.scss",
                "static/fonts/font-awesome-4.7.0/scss/font-awesome.scss",
                "static/fonts/font-awesome-4.7.0/scss/font-awesome.scss",
                "static/fonts/font-awesome-4.7.0/scss/_rotated-flipped.scss",
                "static/fonts/font-awesome-4.7.0/scss/_rotated-flipped.scss",
                "static/fonts/font-awesome-4.7.0/scss/_path.scss",
                "static/fonts/font-awesome-4.7.0/scss/_path.scss",
                "static/fonts/font-awesome-4.7.0/scss/_list.scss",
                "static/fonts/font-awesome-4.7.0/scss/_list.scss",
                "static/fonts/font-awesome-4.7.0/scss/_screen-reader.scss",
                "static/fonts/font-awesome-4.7.0/scss/_screen-reader.scss",
                "static/fonts/font-awesome-4.7.0/scss/_larger.scss",
                "static/fonts/font-awesome-4.7.0/scss/_larger.scss",
                "static/fonts/font-awesome-4.7.0/scss/_core.scss",
                "static/fonts/font-awesome-4.7.0/scss/_core.scss",
                "static/fonts/font-awesome-4.7.0/scss/_mixins.scss",
                "static/fonts/font-awesome-4.7.0/scss/_mixins.scss",
                "static/fonts/font-awesome-4.7.0/scss/_animated.scss",
                "static/fonts/font-awesome-4.7.0/scss/_animated.scss",
                "static/fonts/font-awesome-4.7.0/scss/_icons.scss",
                "static/fonts/font-awesome-4.7.0/scss/_icons.scss",
                "static/fonts/font-awesome-4.7.0/scss/_fixed-width.scss",
                "static/fonts/font-awesome-4.7.0/scss/_fixed-width.scss",
                "static/fonts/font-awesome-4.7.0/scss/_bordered-pulled.scss",
                "static/fonts/font-awesome-4.7.0/scss/_bordered-pulled.scss",
                "static/fonts/font-awesome-4.7.0/HELP-US-OUT.txt",
                "static/fonts/font-awesome-4.7.0/HELP-US-OUT.txt",
                "static/fonts/font-awesome-4.7.0/fonts/fontawesome-webfont.svg",
                "static/fonts/font-awesome-4.7.0/fonts/fontawesome-webfont.svg",
                "static/fonts/font-awesome-4.7.0/fonts/FontAwesome.otf",
                "static/fonts/font-awesome-4.7.0/fonts/FontAwesome.otf",
                "static/fonts/font-awesome-4.7.0/fonts/fontawesome-webfont.woff2",
                "static/fonts/font-awesome-4.7.0/fonts/fontawesome-webfont.woff2",
                "static/fonts/font-awesome-4.7.0/fonts/fontawesome-webfont.ttf",
                "static/fonts/font-awesome-4.7.0/fonts/fontawesome-webfont.ttf",
                "static/fonts/font-awesome-4.7.0/fonts/fontawesome-webfont.woff",
                "static/fonts/font-awesome-4.7.0/fonts/fontawesome-webfont.woff",
                "static/fonts/font-awesome-4.7.0/fonts/fontawesome-webfont.eot",
                "static/fonts/font-awesome-4.7.0/fonts/fontawesome-webfont.eot",
                "static/vendor/animsition/css/animsition.css",
                "static/vendor/animsition/css/animsition.css",
                "static/vendor/animsition/css/animsition.min.css",
                "static/vendor/animsition/css/animsition.min.css",
                "static/vendor/animsition/js/animsition.js",
                "static/vendor/animsition/js/animsition.js",
                "static/vendor/animsition/js/animsition.min.js",
                "static/vendor/animsition/js/animsition.min.js",
                "static/vendor/animate/animate.css",
                "static/vendor/animate/animate.css",
                "static/vendor/bootstrap/css/bootstrap.min.css",
                "static/vendor/bootstrap/css/bootstrap.min.css",
                "static/vendor/bootstrap/css/bootstrap-reboot.min.css.map",
                "static/vendor/bootstrap/css/bootstrap-reboot.min.css.map",
                "static/vendor/bootstrap/css/bootstrap.css",
                "static/vendor/bootstrap/css/bootstrap.css",
                "static/vendor/bootstrap/css/bootstrap-grid.css.map",
                "static/vendor/bootstrap/css/bootstrap-grid.css.map",
                "static/vendor/bootstrap/css/bootstrap-grid.min.css",
                "static/vendor/bootstrap/css/bootstrap-grid.min.css",
                "static/vendor/bootstrap/css/bootstrap.css.map",
                "static/vendor/bootstrap/css/bootstrap.css.map",
                "static/vendor/bootstrap/css/bootstrap.min.css.map",
                "static/vendor/bootstrap/css/bootstrap.min.css.map",
                "static/vendor/bootstrap/css/bootstrap-reboot.min.css",
                "static/vendor/bootstrap/css/bootstrap-reboot.min.css",
                "static/vendor/bootstrap/css/bootstrap-reboot.css",
                "static/vendor/bootstrap/css/bootstrap-reboot.css",
                "static/vendor/bootstrap/css/bootstrap-grid.css",
                "static/vendor/bootstrap/css/bootstrap-grid.css",
                "static/vendor/bootstrap/css/bootstrap-grid.min.css.map",
                "static/vendor/bootstrap/css/bootstrap-grid.min.css.map",
                "static/vendor/bootstrap/css/bootstrap-reboot.css.map",
                "static/vendor/bootstrap/css/bootstrap-reboot.css.map",
                "static/vendor/bootstrap/js/popper.js",
                "static/vendor/bootstrap/js/popper.js",
                "static/vendor/bootstrap/js/tooltip.js",
                "static/vendor/bootstrap/js/tooltip.js",
                "static/vendor/bootstrap/js/bootstrap.js",
                "static/vendor/bootstrap/js/bootstrap.js",
                "static/vendor/bootstrap/js/bootstrap.min.js",
                "static/vendor/bootstrap/js/bootstrap.min.js",
                "static/vendor/bootstrap/js/popper.min.js",
                "static/vendor/bootstrap/js/popper.min.js",
                "static/vendor/css-hamburgers/hamburgers.css",
                "static/vendor/css-hamburgers/hamburgers.css",
                "static/vendor/css-hamburgers/hamburgers.min.css",
                "static/vendor/css-hamburgers/hamburgers.min.css",
                "static/vendor/jquery/jquery-3.2.1.min.js",
                "static/vendor/jquery/jquery-3.2.1.min.js",
                "static/vendor/countdowntime/countdowntime.js",
                "static/vendor/countdowntime/countdowntime.js",
                "static/vendor/daterangepicker/moment.js",
                "static/vendor/daterangepicker/moment.js",
                "static/vendor/daterangepicker/daterangepicker.js",
                "static/vendor/daterangepicker/daterangepicker.js",
                "static/vendor/daterangepicker/daterangepicker.css",
                "static/vendor/daterangepicker/daterangepicker.css",
                "static/vendor/daterangepicker/moment.min.js",
                "static/vendor/daterangepicker/moment.min.js",
                "static/vendor/select2/select2.min.js",
                "static/vendor/select2/select2.min.js",
                "static/vendor/select2/select2.min.css",
                "static/vendor/select2/select2.min.css",
                "static/vendor/select2/select2.js",
                "static/vendor/select2/select2.js",
                "static/vendor/select2/select2.css",
                "static/vendor/select2/select2.css",
                "static/vendor/perfect-scrollbar/perfect-scrollbar.min.js",
                "static/vendor/perfect-scrollbar/perfect-scrollbar.min.js",
                "static/vendor/perfect-scrollbar/perfect-scrollbar.css",
                "static/vendor/perfect-scrollbar/perfect-scrollbar.css",
            );
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
    static_response!(Box::leak(
        file.to_str().unwrap().to_string().into_boxed_str()
    ))
}
