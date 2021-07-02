struct Dog {
    name: String,
}

struct Cat {
    age: u8,
}

enum Animals {
    Dog(Dog),
    Cat(Cat),
}

fn classify(animal: Animals) {
    match animal {
        Animals::Cat(Cat {age}) => println!("A cat that's {} years old", age),
        Animals::Dog(d) => println!("A dog named {}", d.name),
    }
}

fn number(x: i32) {
    match x {
        1 => println!("one"),
        2 | 3 => println!("two or three"),
        4..=i32::MAX => println!("four or bigger"),
        _ => println!("anything"),
    }
}

fn main() {
    for x in 0..6 { number(x)  }
    classify(Animals::Dog(Dog {
        name: "Fido".to_string(),
    }));
    classify(Animals::Cat(Cat { age: 3 }));
}
