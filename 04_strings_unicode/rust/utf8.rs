fn main() {
    let ascii = String::from("Hello");
    println!(
        "'{}': length: {} chars: {} memsize: {}",
        ascii,
        ascii.len(),
        ascii.chars().count(),
        std::mem::size_of::<String>() + ascii.len()
    );
    println!("{:?}", ascii.as_bytes());
    println!("after e: {:?}", &ascii[2..]);

    println!("---");

    let uni = String::from("Héllö");
    println!(
        "'{}': length: {} chars: {} memsize: {}",
        uni,
        uni.len(),
        uni.chars().count(),
        std::mem::size_of::<String>() + uni.len()
    );
    println!("{:?}", uni.as_bytes());
    println!("after é: {:?}", &uni[2..]);
}
