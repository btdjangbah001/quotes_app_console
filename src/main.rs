use std::fs;
use rand::{Rng, thread_rng};
use quotes_app::functions::*;

fn main() {
    let contents = fs::read_to_string("quotes.csv").unwrap();

    let mut new_con = convert_to_vec(&contents);
    new_con.remove(0);
    
    let quotes = create_quotes(&new_con);
    
    loop {
        let num = thread_rng().gen_range(0..quotes.len());
        display_quote(&quotes[num]);

        let choice = read_input("Do you want another quote? Y/N");
        
        if choice != "Y" && choice != "y" {
            break 
        }
    }
}