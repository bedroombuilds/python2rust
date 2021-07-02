mod other;

pub use self::other::otherstuff;
pub fn bar() {
    dbg!("bar");
    otherstuff();
}
