use std::net::{TcpListener, SocketAddrV4, Ipv4Addr,};
use std::io::Error;

pub fn recieve(port: u16,file_name: String) -> Result<(), Error>{
    let loopback = Ipv4Addr::new(127, 0, 0, 1);
    let socket = SocketAddrV4::new(loopback, port);
    let listener = TcpListener::bind(socket)?;
    let port = listener.local_addr()?;
    let (mut tcp_stream, addr) = listener.accept()?;
    
    Ok(())
}