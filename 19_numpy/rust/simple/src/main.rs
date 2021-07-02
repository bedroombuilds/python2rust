use ndarray::prelude::*;
use std::f64::consts::PI;

fn main() -> Result<(), ndarray::ShapeError> {
    let a: Array1<f64> = array![0., 30., 45., 60., 90.];

    println!("angles {}", a);
    println!("sine(a) {}", (a * PI / 180_f64).map(|x| x.sin()));

    let a = Array::from_shape_vec((3, 3), Array1::range(0., 9., 1.).to_vec())?;
    let b = array![10., 10., 10.];

    println!("a: {}", &a);
    println!("b: {}", &b);
    println!("a * 2 {}", &a * 2.);
    println!("a + b {}", &a + &b); // & makes an ArrayView, avoiding move
    println!("a * b {}", &a * &b);
    println!("average(a) {}", a.sum() / a.len() as f64);
    println!("mean(b) {}", b.mean().unwrap());
    Ok(())
}
