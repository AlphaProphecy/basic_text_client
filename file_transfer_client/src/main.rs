
mod reciever;
mod sender;



//use bytes::BytesMut;
//use futures::{Future, Stream};
//use tokio::codec::{BytesCodec, FramedRead};
//use tokio::runtime::Runtime;
use std::env;

// Invoke as echo <interface name>
fn main(){
    let mode_string = match env::args().nth(1) {
        Some(value) => value,
        None => "send".to_string(),
    };

    let path_string = match env::args().nth(2) {
        Some(value) => value,
        None => "test.txt".to_string(),
    };

    let ip_string = match env::args().nth(3) {
        Some(value) => value,
        None => "127.0.0.1".to_string(),
    };

    if mode_string == "send"{
        sender::send(path_string, ip_string);

    } else if mode_string == "recieve" {
        match reciever::recieve(5555,path_string){
            Ok(_) => println!("Recieved"),
            Err(why) => panic!("Error recieving: {}", why)
        }
        
    } else {
        println!("Nice work");
        
    }

}
