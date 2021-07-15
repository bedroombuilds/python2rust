use futures::{SinkExt, StreamExt};
use tokio::io::AsyncBufReadExt;
use tokio::sync::mpsc;

use tokio_tungstenite::connect_async;
use tungstenite::protocol::Message;

#[tokio::main]
async fn main() {
    let connect_addr = "ws://localhost:8000/chat";
    let url = url::Url::parse(&connect_addr).unwrap();
    let (mut ws_stream, _) = connect_async(url).await.expect("Failed to connect");
    println!("WebSocket handshake has been successfully completed");

    let (mut tx_stdin, mut rx) = mpsc::channel::<String>(10);
    // read from stdin
    let stdin_loop = async move {
        loop {
            let mut line = String::new();
            let mut buf_stdin = tokio::io::BufReader::new(tokio::io::stdin());
            buf_stdin.read_line(&mut line).await.unwrap();
            tx_stdin.send(line.trim().to_string()).await.unwrap();
            if line.trim() == "/exit" {
                break;
            }
        }
    };
    tokio::task::spawn(stdin_loop);
    // handle websocket messages
    loop {
        tokio::select! {
            ws_msg = ws_stream.next() => {
                match ws_msg {
                    Some(msg) => match msg {
                        Ok(msg) => match msg {
                            Message::Binary(x) => println!("binary {:?}", x),
                            Message::Text(x) => println!("{}", x),
                            Message::Ping(x) => println!("Ping {:?}", x),
                            Message::Pong(x) => println!("Pong {:?}", x),
                            Message::Close(x) => println!("Close {:?}", x),
                        },
                        Err(_) => {println!("server went away"); break;}
                        },
                    None => {println!("no message"); break;},
                }
            },
            stdin_msg = rx.next() => {
                match stdin_msg {
                    Some(msg) => {
                        let _ = ws_stream.send(Message::Text(msg)).await;
                    },
                    None => break
                }
            }
        };
    }
    // Gracefully close connection by Close-handshake procedure
    let _ = ws_stream.send(Message::Close(None)).await;
    let close = ws_stream.next().await;
    println!("server close msg: {:?}", close);
    assert!(ws_stream.next().await.is_none());
    let _ = ws_stream.close(None).await;
}
