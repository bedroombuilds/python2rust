use serde_json::json;
use warp::Filter;

#[tokio::main]
async fn main() {
    let directory = warp::path("static").and(warp::fs::dir("../../static/"));

    let opt_name = warp::path::param::<String>()
        .map(Some)
        .or_else(|_| async {
            Ok::<(Option<String>,), std::convert::Infallible>((None,)) });
    let hello = warp::path("hello")
        .and(opt_name)
        .map(|name: Option<String>| {
            let msg = format!(
                "Hello, {}!",
                name.unwrap_or("World".to_string()));
            warp::reply::json(&json!({ "message": msg }))
        });
    let root = warp::path::end()
        .and(warp::get())
        .map(|| warp::reply::json(&json!({"message": "Hello, World?"})));

    let items = warp::path!("items" / i32)
        .map(|id| warp::reply::json(&json!({ "item_id": id })));

    let routes = directory.or(hello).or(items).or(root);

    println!("Serving on localhost:3030");
    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}
