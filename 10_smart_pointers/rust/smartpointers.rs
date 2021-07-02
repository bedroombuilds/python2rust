use std::error;
use std::rc::Rc;

type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

struct List {
    next: Option<Box<List>>,
    value: i32,
}

fn main() -> Result<()> {
    let a = Box::new(5);
    println!("deref of box {:?} yields {}", a, *a);
    let mut l = List {
        next: None,
        value: 1,
    };
    l.next = Some(Box::new(List {
        next: None,
        value: 2,
    }));
    println!("list value {}", l.value);

    let b = Rc::new(*a);
    let _c = Rc::clone(&b); // increase reference count (cheap)
    println!("{} refcount: {}", &b, Rc::strong_count(&b));
    // if b is not a Rc type this is potentially deep-copying (expensive)
    // avoid this notation when doing refcount clones
    let _d = b.clone();
    println!("{} refcount: {}", &b, Rc::strong_count(&b));
    Ok(())
}
