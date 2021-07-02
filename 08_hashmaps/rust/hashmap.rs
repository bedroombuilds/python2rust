use std::collections::HashMap; // also look at BTreeMap

fn main() {
    let literal: HashMap<_, _> = vec![("key", "value"), ("blah", "blubb")]
        .into_iter()
        .collect();
    println!("{:?}", literal);

    let mut mutable = HashMap::new();
    mutable.insert("one", 1);
    mutable.insert("two", 2);
    mutable.remove("one");
    println!("{:?}", mutable.get("one"));
    println!("{:?}", mutable.get("two"));

    mutable.insert("three", 3);
    for (k, v) in &mutable {
        println!("{}: {}", k, v);
    }
}
