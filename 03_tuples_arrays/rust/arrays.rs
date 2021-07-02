/// demo of arrays

fn main() {
    let xs = [1, 2, 3, 4, 5];

    let ys: [i32; 500] = [0; 500];
    let _ys = [0_u64; 500];

    println!("first element of the array: {}", xs[0]);
    println!("second element of the array: {}", xs[1]);

    println!("small size: {}", xs.len());
    println!("big size: {}", ys.len());

    println!("borrow a section of the array as a slice {:?}", &xs[1..3]);

    println!("{}", xs[5]);
}
