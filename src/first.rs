use simplebase::engine::*;

pub fn init() {
    let database = new_empty_database();
    database.save_database("issue.txt");
    let database = new_empty_database();
    database.save_database("book.txt");
}
