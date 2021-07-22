#![feature(type_ascription)]
use serde_json::json;

fn main() {
    let o_json = r#"["foo", {"bar": ["baz", null, 1.0, 2]}]"#;
    let o: serde_json::Value = serde_json::from_str(&o_json).unwrap();
    println!("{:?}", &o);

    let o = json!(["foo", {"bar": ("baz", None:Option<i32>, 1.0, 2)}]);
    println!("{:?}", &o);

    let s = &o.to_string();
    println!("{}", &s);
}
