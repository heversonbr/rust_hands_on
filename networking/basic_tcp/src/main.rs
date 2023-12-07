use std::net::TcpListener;

fn main() {

    let listener : TcpListener = TcpListener::bind("0.0.0.0:8888").expect("Couldn't bind the address/port");

    match listener.accept() {
        Ok((_socket, addr)) => println!("Connection from: {addr:?}"),
        Err(e) => println!("couldn't get client: {e:?}"),
    }

    // you can use 'nc 0.0.0.0 8888' to connect a client to this TCP socket
    // after establishing the connection and printing the address the program will finish.

}
