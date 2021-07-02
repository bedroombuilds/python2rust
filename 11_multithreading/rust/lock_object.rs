use std::sync::{Arc, Mutex};

fn main() {
    let x = vec![0, 0];
    let data = Arc::new(Mutex::new(x));
    let mut handles = vec![];

    for _ in 0..10 {
        let mydata = data.clone();
        let h = std::thread::spawn(move || {
            let mut y = mydata.lock().unwrap();
            for _ in 0..100_000 {
                y[0] += 1;
            }
        });
        handles.push(h);
    }
    for h in handles {
        h.join().unwrap();
    }
    println!("{:?}", *data.lock().unwrap());
}
