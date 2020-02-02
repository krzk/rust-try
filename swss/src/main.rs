use std::net::TcpListener;
use std::thread::spawn;
use tungstenite::server::accept;

fn main() {
    let server = TcpListener::bind("0.0.0.0:8081").unwrap();
    for stream in server.incoming() {
        spawn (move || {
            println!("Incoming connection");
            let mut websocket = accept(stream.unwrap()).unwrap();
            println!("Incoming connection accepted");
            loop {
                let msg = websocket.read_message().unwrap();

                if msg.is_binary() || msg.is_text() {
                    println("Msg: {}", msg);
                    websocket.write_message(msg);
                }
            }
        });
    }
}
