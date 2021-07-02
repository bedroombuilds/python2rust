fn do_something(x: u32) -> u32 {
    fn do_some_other(x: u32) -> u32 {
        x * 3
    }
    do_some_other(x)
}

fn main() {
    println!("{}", do_something(3));
}
