use derive_builder::Builder;

#[derive(Debug)]
struct User {
    id: i32,
    email: String,
    first_name: Option<String>,
    last_name: Option<String>,
}

struct UserBuilder {
    id: i32,
    email: String,
    first_name: Option<String>,
    last_name: Option<String>,
}

impl UserBuilder {
    fn new(id: impl Into<i32>, email: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            email: email.into(),
            first_name: None,
            last_name: None,
        }
    }

    fn first_name(mut self, first_name: impl Into<String>) -> Self {
        self.first_name = Some(first_name.into());
        self
    }

    #[allow(dead_code)]
    fn last_name(mut self, last_name: impl Into<String>) -> Self {
        self.last_name = Some(last_name.into());
        self
    }

    fn build(self) -> User {
        let Self {
            id,
            email,
            first_name,
            last_name,
        } = self;
        User {
            id,
            email,
            first_name,
            last_name,
        }
    }
}

impl User {
    fn builder(id: impl Into<i32>, email: impl Into<String>) -> UserBuilder {
        UserBuilder::new(id, email)
    }

    fn complete(&self) -> bool {
        self.last_name.is_some()
            && self.id > 0
            && !self.email.is_empty()
            && self.first_name.is_some()
    }
}

#[derive(Default, Builder, Debug)]
#[builder(setter(into))]
struct Channel {
    #[builder(default = "0")]
    token: i32,
    #[builder(default = r#""42".into()"#)]
    special_info: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let bob = User::builder(13, "bob@example.com")
        .first_name("Bob")
        .build();

    println!("complete? {}", bob.complete());
    println!("bob_the_builder = {:#?}\n\n", bob);

    let c1 = ChannelBuilder::default()
        .special_info("84")
        .build()
        .unwrap();
    let c2 = ChannelBuilder::default()
        .token(4321)
        .build()?;
    println!("{:?} {:?}", c1, c2);
    println!("{} {}", c1.token, c2.special_info);
    Ok(())
}
