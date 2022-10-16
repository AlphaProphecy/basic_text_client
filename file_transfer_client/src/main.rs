mod receiver;
mod sender;

use std::env;

fn main() {
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

    let port = ip_string.split(":").nth(1).expect("You need a port").parse::<u16>().expect("The port is ill formatted");

    if mode_string == "send" {
        sender::send(path_string, ip_string);
    } else if mode_string == "receive" {
        receiver::receive(port, path_string);
    } else {
        println!("Nice work");
    }
}
