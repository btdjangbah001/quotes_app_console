use std::fs::File;
use std::io::{stdin, BufReader, Read};
use std::path::Path;
use crate::quote::Quote;

pub fn read_file(p: &str) -> String {
    let path = Path::new(p);
    let file = File::open(&path).expect("File not found!");
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader
        .read_to_string(&mut contents)
        .expect("File is empty");
    contents
}

pub fn convert_to_vec(contents: &str) -> Vec<&str> {
    contents.split("\n").into_iter().collect::<Vec<_>>()
}

pub fn create_quotes<'a>(q: &'a Vec<&str>) -> Vec<Quote<'a>> {
    let mut quotes = Vec::<Quote>::new();
    for quote in q.iter() {
        let quote_tup = quote.split_once(",");
        quotes.push(Quote::new(quote_tup.unwrap().0, quote_tup.unwrap().1));
    }
    quotes
}

pub fn read_input(msg: &str) -> String {
    println!("{}", msg);
    let mut line = String::new();
    stdin().read_line(&mut line).unwrap();
    line.trim().to_string()
}

pub fn display_quote(q: &Quote) {
    println!("-------------------------------------------------------------");
    println!();
    println!("Quote: {}", q.quote);
    println!("Author: {}", q.author);
    println!();
    println!("-------------------------------------------------------------");
}
