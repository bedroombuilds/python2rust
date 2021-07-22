use minidom::Element;

const NS: &str = "";
fn main() -> Result<(), Box<dyn std::error::Error + 'static>> {
    println!("minidom");
    let data = std::fs::read_to_string("../../cd-list.xml")?;
    let mut root: Element = data.parse().unwrap();

    for child in root.children() {
        println!("{:?}", child);
    }

    for child in root.children().filter(|e| e.is("title", NS)) {
        println!("title {:?}", child.text());
    }
    for child in root.children().filter(|e| e.is("cd", NS)) {
        for tl in child.children().filter(|e| e.is("track-list", NS)) {
            for track in tl.children().filter(|e| e.is("track", NS)) {
                println!("track {:?}", track.attr("no"));
            }
        }
    }

    let newcd = Element::builder("cd", "")
        .append(
            Element::builder("artist", "")
                .append("bob")
                .attr("country", "DE"),
        )
        .append(Element::builder("title", "").append("Song for Alice"))
        .build();
    root.append_child(newcd);
    let mut buf = Vec::<u8>::new();
    root.write_to(&mut buf).unwrap();
    println!("{}", String::from_utf8(buf).unwrap());
    Ok(())
}
