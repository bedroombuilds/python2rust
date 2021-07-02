use geo::{Area, Circle, Perimeter, Square};

fn main() {
    let c = Circle {
        x: 0.0,
        y: 1.0,
        radius: 3.0,
    };
    let s = Square {
        x: 0.0,
        y: 1.0,
        sidelen: 2.0,
    };
    println!("The circles area is: {}", c.area());
    println!("The circles perimeter is: {}", c.circumference());
    println!("The squares area is: {}", s.area());
}
