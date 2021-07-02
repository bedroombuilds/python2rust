fn fizz_buzz(x: i32) -> String {
    match (x % 3, x % 5) {
        (0, 0) => "FizzBuzz".into(),
        (0, _) => "Fizz".into(),
        (_, 0) => "Buzz".into(),
        (_, _) => format!("{}", x),
    }
}

fn vec_fizz_buzz(n: i32) -> Vec<String> {
    let mut v = Vec::new();
    for i in 1..(n + 1) {
        v.push(dbg!(fizz_buzz(i)));
    }
    v
}

fn main() {
    println!("{:?}", vec_fizz_buzz(1));
}
