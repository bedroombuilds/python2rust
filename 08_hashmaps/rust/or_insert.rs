use std::collections::HashMap;

fn main() {
    let mut d: HashMap<String, Vec<i32>> = HashMap::new();

    d.entry("banana".to_string())
        .or_insert_with(Vec::new)
        .push(0);
    println!("{:?}", d);
    for k in d.keys() {
        println!("{:?}", k);
    }
    for v in d.values() {
        println!("{:?}", v);
    }

    let mut td: HashMap<(i32, i32), Vec<i32>> = HashMap::new();
    td.insert((0, 1), vec![]);
}
