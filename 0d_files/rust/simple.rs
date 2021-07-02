use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn main() -> io::Result<()> {
    let args: Vec<String> = std::env::args().collect();

    let file = File::open(&args[1])?;
    let reader = BufReader::new(file);

    let mut found = 0usize;
    for line in reader.lines() {
        let line = line?;
        let elems: Vec<&str> = line.split(',').collect();
        if elems[1] == "Tanzania" {
            found += 1;
        }
    }
    println!("found {} sales for Tanzania", found);
    Ok(())
}
