struct A {
    x: i32,
    y: i32,
}

struct B {
    members: static Vec<i32>,
}

fn main() {
    let mut a = A { x: 1, y: 2 };
    a.x += 2;
    let a_updated = A { y: 4, ..a };
    println!("a.x {} a_updated.x {}", a.x, a_updated.x);
    println!("a.y {} a_updated.y {}", a.y, a_updated.y);
}
