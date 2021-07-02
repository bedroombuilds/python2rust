//! demo of vectors

fn main() {
    let mut xs = vec![1_i32, 2, 3];
    println!("Initial vector: {:?}", xs);

    println!("Push 4 into the vector");
    xs.push(4);
    println!("Vector: {:?}", xs);

    println!("Vector length: {}", xs.len());

    println!("Second element: {}", xs[1]);

    println!("Pop last element: {:?}", xs.pop());

    println!("Fourth element: {}", xs[3]);

    println!("Contents of xs:");
    for x in xs {
        // borrow or don't use into_iter()
        println!("> {}", x);
    }

    /*
    for (i, x) in xs.iter().enumerate() {
        println!("In position {} we have value {}", i, x);
    }

    for x in xs.iter_mut() {
        *x *= 3;
    }
    let xs: Vec<i32> = xs.iter().map(|x| x * 3).collect();
    println!("Updated vector: {:?}", xs);

    let collected_iterator: Vec<i32> = (0..10).collect();
    println!("Collected (0..10) into: {:?}", collected_iterator);
    */
}
