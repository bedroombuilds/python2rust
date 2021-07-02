//! demoing tuple use.

fn reverse(pair: (i32, bool)) -> (bool, i32) {
    let (integer, boolean) = pair;

    (boolean, integer)
}

#[derive(Debug)]
struct Matrix(f32, f32, f32, f32);

fn main() {
    let many_types = (1u8, 2u16, 'a', true);

    println!("first value: {}", many_types.0);
    println!("second value: {}", many_types.1);

    let tuple_of_tuples = ((1u8, 2u16, 2u32), (4u64, -1i8), -2i16);
    println!("tuple of tuples: {:?}", tuple_of_tuples);

    let pair = (1, true);
    println!("pair is {:?}", pair);
    println!("the reversed pair is {:?}", reverse(pair));

    println!("one element tuple: {:?}", (5u32,));
    println!("just an integer: {:?}", (5u32));

    let tuple = (1, "hello", 4.5, true);
    let (a, b, c, d) = tuple;
    println!("{:?}, {:?}, {:?}, {:?}", a, b, c, d);

    let matrix = Matrix(1.1, 1.2, 2.1, 2.2);
    println!("{:?}", matrix);
}
