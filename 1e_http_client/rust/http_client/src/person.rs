use serde_derive::Deserialize;

pub static URL: &str = "https://pipl.ir/v1/getPerson";

pub fn summary(data: serde_json::Value) -> String {
    let p = &data["person"]["personal"];
    format!(
        "{} {}, {}, {}",
        p["name"].as_str().unwrap(),
        p["last_name"].as_str().unwrap(),
        p["age"],
        p["country"].as_str().unwrap(),
    )
}

#[derive(Deserialize)]
pub struct Data {
    pub person: Person,
}

#[derive(Deserialize)]
pub struct Person {
    pub education: serde_json::Value,
    pub marriage: Marriage,
    pub online_info: serde_json::Value,
    pub personal: serde_json::Value,
    pub work: serde_json::Value,
}

#[derive(Deserialize)]
pub struct Marriage {
    pub married: bool,
    pub children: Option<usize>,
    pub spouse_name: Option<String>,
}

impl std::fmt::Display for Data {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let p = &self.person.personal;
        write!(
            f,
            "Person: {} {}, {}, {}",
            p["name"].as_str().unwrap(),
            p["last_name"].as_str().unwrap(),
            p["age"],
            p["country"].as_str().unwrap(),
        )
    }
}
