fn main() {
    let mut data: u32 = 0;
    let mut handles = vec![];

    for _ in 0..10 {
        let h = std::thread::spawn(move || {
            for _ in 0..100_000 {
                data += 1;
            }
            println!("{}", data);
        });

        handles.push(h);
    }
    for h in handles {
        h.join().unwrap();
    }
    println!("{}", data);
}
