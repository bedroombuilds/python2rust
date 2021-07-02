mod foo; // this loads the file foo.rs

fn main() {
    dbg!("main");
    foo::bar();
    foo::otherstuff();
    //foo::other::otherstuff();
}
