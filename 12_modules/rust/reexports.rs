mod family {
    mod parents {
        pub fn call_mom() {}
        pub fn call_brother() {}
    }

    // re-export
    pub use self::parents::{call_brother, call_mom};
}

fn main() {
    family::call_mom();
    family::call_brother();
    //family::parents::call_brother();
}
