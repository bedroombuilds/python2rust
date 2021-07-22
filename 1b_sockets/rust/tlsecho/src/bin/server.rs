use std::fs;
use std::io::{BufRead, BufReader, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::Arc;
use std::thread;

fn open_cert_file<F, T>(file: &str, method: F) -> Vec<T>
where
    F: Fn(&mut dyn BufRead) -> Result<Vec<T>, ()>,
{
    let certfile = fs::File::open(file).unwrap();
    let mut reader = BufReader::new(certfile);
    method(&mut reader).unwrap()
}

fn main() {
    let mut config = rustls::ServerConfig::new(rustls::NoClientAuth::new());
    let certs = open_cert_file("../../certchain.pem", rustls::internal::pemfile::certs);
    let key = open_cert_file(
        "../../private.key",
        rustls::internal::pemfile::pkcs8_private_keys,
    )
    .pop()
    .unwrap();
    let listener = TcpListener::bind("127.0.0.1:12321").unwrap();
    config.set_single_cert(certs, key).unwrap();
    println!("server is running on 127.0.0.1:12321 ...");

    for socket in listener.incoming() {
        let socket = socket.unwrap();
        let config = config.clone();
        thread::spawn(move || handle_client(socket, Arc::new(config)));
    }
}

fn handle_client(mut socket: TcpStream, config: Arc<rustls::ServerConfig>) {
    let mut sess = rustls::ServerSession::new(&config);
    let stream = rustls::Stream::new(&mut sess, &mut socket);
    let mut reader = BufReader::new(stream);
    loop {
        let mut buf = String::new();
        if reader.read_line(&mut buf).is_err() {
            break;
        }
        if !buf.is_empty() {
            reader.get_mut().write_all(buf.as_bytes()).unwrap();
        }
    }
}
