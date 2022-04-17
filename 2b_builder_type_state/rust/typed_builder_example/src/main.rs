#![allow(dead_code)]
use typed_builder::TypedBuilder;

#[derive(Debug, TypedBuilder)]
struct User {
    id: String,
    email: String,
    #[builder(default)]
    first_name: Option<String>,
    #[builder(default, setter(into, strip_option))]
    last_name: Option<String>,
}

fn main() {
    let user = User::builder()
        .id("42".into())
        //.email("bedroombuilds@example.com".into())
        .first_name(Some("Bedroom".into()))
        //.last_name("Builds")
        .build();
    dbg!(user);
}
