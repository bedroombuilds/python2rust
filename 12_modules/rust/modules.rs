use std::io as sio;
use std::rc::{Rc as RefCounted, Weak};

mod foo {
    pub fn bar() {}
}

fn main() -> sio::Result<()> {
    let x = RefCounted::new(std::i32::MAX);
    let _y: Weak<i32> = RefCounted::downgrade(&x);
    foo::bar();
    Ok(())
}
