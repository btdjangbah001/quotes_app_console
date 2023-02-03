use std::io::stdin;
use crate::quote::Quote;

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
