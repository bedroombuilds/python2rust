use xmltree::{Element, XMLNode};

fn main() -> Result<(), Box<dyn std::error::Error + 'static>> {
    println!("Hello, world!");
    let data = std::fs::read_to_string("../../cd-list.xml")?;
    let mut root = Element::parse(data.as_bytes()).unwrap();
    for child in &root.children {
        println!("child {:?}", child);
    }
    let titles = &root.get_child("title").unwrap();
    for title in &titles.children {
        println!("title {:?}", title.as_text());
        println!("title {:?}", &titles.get_text());
    }
    let cd = &root.get_child("cd").unwrap();
    let track_list = cd.get_child("track-list").unwrap();
    let track = track_list.get_child("track").unwrap();
    println!("track-no {:?}", track.attributes["no"]);

    let mut newcd = Element::new("cd");
    let mut artist = Element::new("artist");
    artist.children.push(XMLNode::Text("Bob".to_owned()));
    artist
        .attributes
        .insert("country".to_owned(), "DE".to_owned());
    let mut title = Element::new("title");
    title
        .children
        .push(XMLNode::Text("Song for Alice".to_owned()));
    newcd.children.push(XMLNode::Element(title));
    root.children.push(XMLNode::Element(newcd));
    let mut buf = Vec::<u8>::new();
    root.write(&mut buf).unwrap();
    println!("{}", String::from_utf8(buf).unwrap());
    Ok(())
}
