use std::time::Instant;

mod person;

fn main() -> Result<(), ureq::Error> {
    let data: person::Data = ureq::get(person::URL)
        .set("User-Agent", "Mozilla/5.0 (Macintosh; M1 Mac OS X 11_9_3)")
        .call()?
        .into_json()?;
    println!("{}", data);

    let now = Instant::now();
    for _ in 0..50 {
        let body: serde_json::Value = ureq::get(person::URL)
            .set("User-Agent", "Mozilla/5.0 (Macintosh; M1 Mac OS X 11_9")
            .call()?
            .into_json()?;
        println!("{}", person::summary(body));
    }
    println!("{:.2}secs runtime", now.elapsed().as_secs_f64());
    Ok(())
}
