use eyre::Result;
use thiserror::Error;

#[derive(Error, Debug)]
enum DoubleError {
    #[error("no first item")]
    EmptyVec,
    #[error("invalid first item, error: '{0}'")]
    Parse(#[from] std::num::ParseIntError),
}

fn double_first(vec: Vec<&str>) -> Result<i32> {
    let first = vec.first().ok_or(DoubleError::EmptyVec)?;
    let parsed = first.parse::<i32>().map_err(|e| DoubleError::Parse(e))?;
    Ok(2 * parsed)
}

fn print(result: Result<i32>) {
    match result {
        Ok(n) => println!("The first doubled is {}", n),
        Err(e) => println!("Error: {}", e),
    }
}

fn main() {
    let numbers = vec!["42", "93", "18"];
    let empty = vec![];
    let strings = vec!["tofu", "93", "18"];

    print(double_first(numbers));
    print(double_first(empty));
    print(double_first(strings));
}
