use std::io::prelude::*;
use std::io::BufReader;
use std::net::TcpStream;

fn main() {
    let mut stream = TcpStream::connect("127.0.0.1:12321").unwrap();

    let len = stream.write(b"hello socket world\n");
    println!("written {:?}", len);
    let mut reader = BufReader::new(stream);
    let mut line = String::new();
    let len = reader.read_line(&mut line);
    println!("{:?} {:?}", &line, len);
}
