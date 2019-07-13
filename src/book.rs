use simplebase::engine::*;
use std::fmt;

struct Row {
    name: String,
    author: String,
    book_code: String,
    total_books: usize,
}

impl fmt::Display for Row {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{} {} {} {}",
            self.name, self.author, self.book_code, self.total_books
        )
    }
}

impl Base for Row {
    fn addb(self) -> (DataType, String) {
        (DataType::Empty, format!("{}", self))
    }
}

pub fn input(name: String, author: String, book_code: String) {
    let mut database = load_hash_database("book.txt");
    let result = database.find(&format!("{} {} {}", name, author, book_code)[..]);
    if result.is_empty() {
        let row = Row {
            name,
            author,
            book_code,
            total_books: 1,
        };
        database.add_record(row);
    } else {
        let index = match result[0].parse::<usize>() {
            Ok(x) => x,
            Err(_) => {
                println!("The index is not a number...");
                println!("vec -> {:?}", result);
                panic!();
            }
        };
        let record = database.get_record(index);
        let record = match record {
            Some(x) => x
                .split_whitespace()
                .map(|x| x.to_string())
                .collect::<Vec<String>>(),
            None => {
                println!("Something's wrong...");
                println!("Unable to get the record...");
                println!("vec -> {:?}", record);
                panic!();
            }
        };
        let row = Row {
            name,
            author,
            book_code,
            total_books: match record[3].parse::<usize>() {
                Ok(x) => x + 1,
                Err(_) => {
                    println!("Something's wrong...");
                    println!("The book no in the record is not a num...");
                    println!("vec -> {:?}", record);
                    panic!();
                }
            },
        };
        database.delete_record(index);
        database.add_record(row);
    }
    database.save_database("book.txt");
}

pub fn add_book(book_code: String) {
    let mut database = load_hash_database("book.txt");
    let result = database.find(&book_code.to_string()[..]);
    let index = match result[0].parse::<usize>() {
        Ok(x) => x,
        Err(_) => {
            println!("The index is not a number...");
            println!("vec -> {:?}", result);
            panic!();
        }
    };
    let record = database.get_record(index);
    let record = match record {
        Some(x) => x
            .split_whitespace()
            .map(|x| x.to_string())
            .collect::<Vec<String>>(),
        None => {
            println!("Something's wrong...");
            println!("Unable to get the record...");
            println!("vec -> {:?}", record);
            panic!();
        }
    };
    let row = Row {
        name: record[0].clone(),
        author: record[1].clone(),
        book_code: record[2].clone(),
        total_books: match record[3].parse::<usize>() {
            Ok(x) => x + 1,
            Err(_) => {
                println!("Something's wrong...");
                println!("The book no in the record is not a num...");
                println!("vec -> {:?}", record);
                panic!();
            }
        },
    };
    database.delete_record(index);
    database.add_record(row);
    database.save_database("book.txt");
}

pub fn remove_book(book_code: String) {
    let mut database = load_hash_database("book.txt");
    let result = database.find(&book_code.to_string()[..]);
    let index = match result[0].parse::<usize>() {
        Ok(x) => x,
        Err(_) => {
            println!("The index is not a number...");
            println!("vec -> {:?}", result);
            panic!();
        }
    };
    let record = database.get_record(index);
    let record = match record {
        Some(x) => x
            .split_whitespace()
            .map(|x| x.to_string())
            .collect::<Vec<String>>(),
        None => {
            println!("Something's wrong...");
            println!("Unable to get the record...");
            println!("vec -> {:?}", record);
            panic!();
        }
    };
    let row = Row {
        name: record[0].clone(),
        author: record[1].clone(),
        book_code: record[2].clone(),
        total_books: match record[3].parse::<usize>() {
            Ok(x) => x - 1,
            Err(_) => {
                println!("Something's wrong...");
                println!("The book no in the record is not a num...");
                println!("vec -> {:?}", record);
                panic!();
            }
        },
    };
    database.delete_record(index);
    database.add_record(row);
    database.save_database("book.txt");
}
