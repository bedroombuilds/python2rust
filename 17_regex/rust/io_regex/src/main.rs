use std::io;
use lazy_static::lazy_static;
use regex::Regex;
mod iso_8601;

/// The recognized date format is YYYY-MM-DD.
fn is_proper_date(text: &str) -> bool {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^(\d{4})-(\d{2})-(\d{2})$").unwrap();
    }
    RE.is_match(text)
}

fn main() {
    println!("Type 'quit' to exit.");
    let mut input = String::new();
    loop {
        io::stdin().read_line(&mut input).unwrap();
        input.truncate(input.len() - 1); // remove trailing newline

        if input == "quit" {
            println!("Good bye");
            break;
        }

        let is_date = is_proper_date(&input);
        //let is_date = iso_8601::is_8601_date(&input);
        println!(
            "{0} is{1}a date",
            input,
            if is_date { " " } else { " not " }
        );
        input.clear();
    }
}
