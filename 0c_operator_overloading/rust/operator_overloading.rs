use std::fmt;
use std::fmt::Write;
use std::ops;

#[derive(Debug, Clone)]
struct MyString(String);

impl fmt::Display for MyString {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl ops::Add<String> for MyString {
    type Output = MyString;

    fn add(self, rhs: String) -> Self::Output {
        println!("> MyString.add({}) was called", &rhs);
        MyString(format!("{}{}", self.0, rhs))
    }
}

impl ops::Add<i32> for MyString {
    type Output = MyString;

    fn add(self, rhs: i32) -> Self::Output {
        println!("> MyString.add({}) was called", &rhs);
        MyString(format!("{}{}", self.0, rhs))
    }
}

impl ops::Add<MyString> for i32 {
    type Output = MyString;

    fn add(self, rhs: MyString) -> Self::Output {
        println!("> i32.add({}) was called", &rhs);
        MyString(format!("{}{}", self, rhs))
    }
}

impl ops::Mul<i32> for MyString {
    type Output = MyString;

    fn mul(self, rhs: i32) -> Self::Output {
        println!("> MyString.mul({}) was called", &rhs);
        let caplen: usize = if rhs <= 0 { 0 } else { rhs as usize };
        let mut temp = String::with_capacity(&self.0.len() * caplen);
        for _ in 0..rhs {
            temp.write_str(&self.0).expect("writing to string failed");
        }
        MyString(temp)
    }
}

fn main() {
    let ms = MyString("Foo".to_string());
    println!("Foo + Bar = {}", ms + "Bar".to_string());
    println!("Foo + -12 = {}", ms + -12_i32);
    println!("100 + Foo = {}", 100_i32 + ms.clone());
    println!("Foo * -12 = {}", ms.clone() * -12_i32);
    println!("Foo * 12 = {}", ms.clone() * 12_i32);
}
