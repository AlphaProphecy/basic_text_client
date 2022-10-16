use std::io::prelude::*;
use std::io::BufReader;
use std::net::{Ipv4Addr, SocketAddrV4, TcpListener, TcpStream};

pub fn receive(port: u16, _file_name: String) {
    let loopback = Ipv4Addr::new(127, 0, 0, 1);
    let socket = SocketAddrV4::new(loopback, port);
    let listener = TcpListener::bind(socket).expect("Failed to accept");
    let port = listener.local_addr().expect("Failed to accept");
    println!("Listing on: {}", port);
    let (tcp_stream, _addr) = listener.accept().expect("Failed to accept");

    handle_client(tcp_stream);

}

fn handle_client(stream: TcpStream) {
    let mut stream = BufReader::new(stream);
    println!("New client logged");

    loop {
        let mut recv_buffer = String::new();
        println!("Before reading");
        let recv_data_size = stream
            .read_line(&mut recv_buffer)
            .expect("Error when reading line");

        if recv_data_size > 0 {
            print!("{}", recv_buffer);
        } else {
            println!("Connection closed");
            // stream has reached EOF
            break;
        }
    }
}
