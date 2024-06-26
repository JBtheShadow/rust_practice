use std::net::TcpListener;

fn main() {
    let addr = "127.0.0.1:7878";
    let listener = TcpListener::bind(addr).unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        println!("Connection established!");
    }
}