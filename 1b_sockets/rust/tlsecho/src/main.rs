use std::sync::Arc;

use std::io::{stdout, Read, Write};
use std::net::TcpStream;

use rustls;
use webpki;
use webpki_roots;

use rustls::Session;

fn main() {
    let hostname = "michael.kefeder.at";
    let host = "127.0.0.1:12321";
    let mut config = rustls::ClientConfig::new();
    config
        .root_store
        .add_server_trust_anchors(&webpki_roots::TLS_SERVER_ROOTS);

    let dns_name = webpki::DNSNameRef::try_from_ascii_str(hostname).unwrap();
    let mut sess = rustls::ClientSession::new(&Arc::new(config), dns_name);
    let mut sock = TcpStream::connect(host).unwrap();
    let mut tls = rustls::Stream::new(&mut sess, &mut sock);
    tls.write_all(b"Hello bob\n").unwrap();
    let ciphersuite = tls.sess.get_negotiated_ciphersuite().unwrap();
    writeln!(
        &mut std::io::stderr(),
        "Current ciphersuite: {:?}",
        ciphersuite.suite
    )
    .unwrap();
    let mut plaintext = [0u8; 1024];
    let num_bytes = tls.read(&mut plaintext).unwrap();
    if num_bytes > 0 {
        stdout().write_all(&plaintext).unwrap();
    }
    println!("{:?}", num_bytes);
}
