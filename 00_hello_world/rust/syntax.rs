//! docstring for module

/// docstring for function
fn multiply(x: i64, y: i64) -> i64 {

    // one line comment
    x * y
    /* multiline comment explaining
     * that for returning a value
     * an expression as last line is enough
     */
}


fn main() {
    let z = multiply(5, 6);
    println!("result: {}", z);
    let txt = format!("result: {z}", z=z);
    println!("{}", txt);
}
