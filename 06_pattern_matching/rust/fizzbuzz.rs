fn main() {
    fn if_buzz(x: i32) {
        if x % 15 == 0 {
            println!("FizzBuzz")
        } else if x % 3 == 0 {
            println!("Fizz")
        } else if x % 5 == 0 {
            println!("Buzz")
        } else {
            println!("{}", x)
        }
    }

    let fizz_buzz = |x| match (x % 3, x % 5) {
        (0, 0) => println!("FizzBuzz"),
        (0, _) => println!("Fizz"),
        (_, 0) => println!("Buzz"),
        (_, _) => println!("{}", x),
    };

    (1..24).into_iter().for_each(fizz_buzz);
}
