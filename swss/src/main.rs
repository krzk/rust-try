//use std::net::TcpListener;
use std::thread::spawn;
use tungstenite::server::accept;

use openssl::ssl::{SslMethod, SslAcceptor, SslStream, SslFiletype};
use std::net::{TcpListener, TcpStream};
use std::sync::Arc;
use std::thread;

fn main() {
    let mut acceptor = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
    // TODO: maybe it's possible to skip some steps? Combine cert into chain?
    //acceptor.set_certificate_chain_file("/home/krzk/secure/ca-chain.cert.pem").unwrap();
    acceptor.set_private_key_file("/home/krzk/secure/internal-development.tls.key.pem", SslFiletype::PEM).unwrap();
    acceptor.set_certificate_file("/home/krzk/secure/internal-development.tls.cert.pem", SslFiletype::PEM).unwrap();
    acceptor.check_private_key().unwrap();
    let acceptor = Arc::new(acceptor.build());

    let server = TcpListener::bind("0.0.0.0:8081").unwrap();
    for stream in server.incoming() {
        match stream {
            Ok(stream) => {
                println!("Incoming connection");
                let acceptor = acceptor.clone();
                spawn (move || {
                    let stream = acceptor.accept(stream).unwrap();
                    //let mut websocket = accept(stream.unwrap()).unwrap();
                    let mut websocket = accept(stream).unwrap();
                    println!("Incoming connection accepted");
                    loop {
                        let msg = websocket.read_message().unwrap();

                        if msg.is_binary() || msg.is_text() {
                            println!("Msg: {}", msg);
                            websocket.write_message(msg);
                        }
                    }
                });
            }
            Err(e) => {
                println!("Error accepting connection");
            }
        }

    }
}
