fn main() {

    let x = 1;
    println!("x: {}", x);
    // binds x again, shadowing the old one from above
    let x = 'i';
    println!("x: {}", x);

    // declare, initialize
    let something;
    let x = 5;
    println!("x, something: {}, {}", x, something);
    something = x * 5;
    println!("x, something: {}, {}", x, something);

    // Mutability
    let y = 0;
    y = y * 2 + x;
    dbg!(y);

    const blah: i32 = 42;
    y *= blah;
    dbg!(y);
}
