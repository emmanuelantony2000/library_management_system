use simplebase::engine::*;

pub fn init() {
    let database = new_empty_database();
    database.save_database("book_issue.txt");
    let database = new_empty_database();
    database.save_database("input_books.txt");
}
