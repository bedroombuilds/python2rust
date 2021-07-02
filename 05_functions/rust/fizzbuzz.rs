fn main() {
    let fizz_buzz = |x| {
        if x % 15 == 0 {
            println!("FizzBuzz")
        } else if x % 3 == 0 {
            println!("Fizz")
        } else if x % 5 == 0 {
            println!("Buzz")
        } else {
            println!("{}", x)
        }
    };
    for i in 1..16 {
        fizz_buzz(i)
    }
    println!("---");
    (1..16).into_iter().for_each(fizz_buzz);
}
