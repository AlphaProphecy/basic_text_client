use std::io;
use std::io::prelude::*;
use std::io::BufWriter;
use std::net::TcpStream;
use std::path::Path;

pub fn send(path_string: String, ip_string: String) {

    let _path = Path::new(&path_string);

    println!("{}", ip_string);
    let stream = TcpStream::connect(ip_string).expect("Failed to connect");
    handle_client(stream);
}

fn handle_client(tcp_stream: TcpStream) {
    let mut stream = BufWriter::new(tcp_stream);
    println!("Sending Data: ");

    loop {
        let mut buffer = String::new();
        let stdin = io::stdin();
        stdin
            .read_line(&mut buffer)
            .expect("Error when reading line");

        if buffer.trim().is_empty() {
            println!("Empty line received closing connection");
            break;
        }

        let _recv_data_size = stream
            .write(&mut buffer.as_bytes())
            .expect("Error when reading line");

        match stream.flush() {
            Ok(_) => continue,
            Err(_) => {
                println!("Remote connection Dropped");
                break;
            }
        }
    }
    drop(stream);
}
