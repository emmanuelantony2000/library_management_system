use crate::*;
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
        let hash = hash!(self.book_code);
        write!(
            f,
            "{}{}{}{}{}{}{}",
            self.name, hash, self.author, hash, self.book_code, hash, self.total_books
        )
    }
}

impl Base for Row {
    fn addb(self) -> (DataType, String) {
        (DataType::Empty, format!("{}", self))
    }
}

pub fn input(name: String, author: String, book_code: String) -> Result<(), ()> {
    let mut database = load_hash_database("book.txt");
    let result = database.find(&book_code[..]);
    if result.is_empty() {
        let row = Row {
            name,
            author,
            book_code,
            total_books: 1,
        };
        database.add_record(row);
        database.save_database("book.txt");
    } else {
        add_book(book_code)?;
    }
    Ok(())
}

pub fn add_book(book_code: String) -> Result<(), ()> {
    let mut database = load_hash_database("book.txt");
    let result = database.find(&book_code[..]);
    if result.len() == 2 {
        let hash = hash!(book_code);
        let record = result[1]
            .split(&hash[..])
            .map(|x| x.to_string())
            .collect::<Vec<String>>();
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
                    return Err(());
                }
            },
        };
        database.delete_record(match result[0].parse::<usize>() {
            Ok(x) => x,
            Err(_) => {
                println!("The index is not a number...");
                println!("vec -> {:?}", result);
                return Err(());
            }
        });
        database.add_record(row);
        database.save_database("book.txt");
        Ok(())
    } else {
        println!("Something's wrong...");
        println!("Cannot find the book...");
        println!("vec -> {:?}", result);
        Err(())
    }
}

pub fn remove_book(book_code: String) -> Result<(), ()> {
    let mut database = load_hash_database("book.txt");
    let result = database.find(&book_code[..]);
    if result.len() == 2 {
        let hash = hash!(book_code);
        let record = result[1]
            .split(&hash[..])
            .map(|x| x.to_string())
            .collect::<Vec<String>>();
        let row = Row {
            name: record[0].clone(),
            author: record[1].clone(),
            book_code: record[2].clone(),
            total_books: match record[3].parse::<usize>() {
                Ok(x) => {
                    if x == 0 {
                        println!("Cannot remove a book when there is none...");
                        return Err(());
                    } else {
                        x - 1
                    }
                }
                Err(_) => {
                    println!("Something's wrong...");
                    println!("The book no in the record is not a num...");
                    println!("vec -> {:?}", record);
                    return Err(());
                }
            },
        };
        database.delete_record(match result[0].parse::<usize>() {
            Ok(x) => x,
            Err(_) => {
                println!("The index is not a number...");
                println!("vec -> {:?}", result);
                return Err(());
            }
        });
        database.add_record(row);
        database.save_database("book.txt");
        Ok(())
    } else {
        println!("Something's wrong...");
        println!("Cannot find the book...");
        println!("vec -> {:?}", result);
        Err(())
    }
}
