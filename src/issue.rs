use super::book;
use crate::*;
use chrono::prelude::*;
use simplebase::engine::*;
use std::fmt;
use time::Duration;

struct Row<T> {
    name: String,
    class: String,
    id: String,
    book_code: String,
    date_taken: T,
    date_return: T,
    date_returned: Option<DateTime<Local>>,
}

impl<T> fmt::Display for Row<T>
where
    T: std::fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let hash = hash!(self.book_code);
        write!(
            f,
            "{}{}{}{}{}{}{}{}{}{}{}{}{}",
            self.name,
            hash,
            self.class,
            hash,
            self.id,
            hash,
            self.book_code,
            hash,
            self.date_taken,
            hash,
            self.date_return,
            hash,
            match self.date_returned {
                Some(x) => x.to_string(),
                None => "NIL".to_string(),
            }
        )
    }
}

impl<T> Base for Row<T>
where
    T: std::fmt::Display,
{
    fn addb(self) -> (DataType, String) {
        (DataType::Empty, format!("{}", self))
    }
}

pub fn issue(name: String, class: String, id: String, book_code: String) -> Result<(), ()> {
    let mut database = load_hash_database("issue.txt");
    let books = load_hash_database("book.txt");
    let books = books.find(&book_code[..]);
    let hash = hash!(book_code);
    if books.is_empty() {
        println!("Book is not present in the system...");
        println!("{:?}", load_hash_database("book.txt"));
        return Err(());
    } else {
        let result = database.find(
            &format!(
                "{}{}{}{}{}{}{}",
                name, hash, class, hash, id, hash, book_code
            )[..],
        );
        if result.is_empty() {
            let row = Row {
                name,
                class,
                id,
                book_code: book_code.clone(),
                date_taken: Local::now(),
                date_return: Local::now() + Duration::days(14),
                date_returned: Option::None,
            };
            database.add_record(row);
            book::remove_book(book_code)?;
        } else {
            let mut index = result
                .iter()
                .step_by(2)
                .map(|x| match x.parse::<usize>() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("Something's wrong...");
                        println!("The index is not a number...");
                        println!("vec -> {:?}", result);
                        panic!();
                    }
                })
                .collect::<Vec<usize>>();
            index.sort();
            let record = database.get_record(index[index.len() - 1]);
            let record = match record {
                Some(x) => x
                    .split(&hash[..])
                    .map(|x| x.to_string())
                    .collect::<Vec<String>>(),
                None => {
                    println!("Something's wrong...");
                    println!("Unable to get the record...");
                    println!("vec -> {:?}", record);
                    return Err(());
                }
            };
            if record[record.len() - 1] == "NIL" {
                let row = Row {
                    name,
                    class,
                    id,
                    book_code: book_code.clone(),
                    date_taken: record[4].clone(),
                    date_return: record[5].clone(),
                    date_returned: Option::Some(Local::now()),
                };
                database.delete_record(index[index.len() - 1]);
                database.add_record(row);
                book::add_book(book_code)?;
            } else {
                let row = Row {
                    name,
                    class,
                    id,
                    book_code: book_code.clone(),
                    date_taken: record[4].clone(),
                    date_return: record[5].clone(),
                    date_returned: Option::None,
                };
                database.add_record(row);
                book::remove_book(book_code)?;
            }
        }
    }
    database.save_database("issue.txt");
    Ok(())
}
