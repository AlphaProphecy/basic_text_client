

use std::env;

use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

// Invoke as echo <interface name>
fn main() {

    let path_string = match env::args().nth(1) {
        Some(value) => value,
        None => "test.txt".to_string(),
    };

    let ip_string = match env::args().nth(1) {
        Some(value) => value,
        None => "127.0.0.1".to_string(),
    };


    let path = Path::new(&path_string);
    let display = path.display();
    
    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", display, why),
        Ok(_) => print!("{} contains:\n{}", display, s),
    };

}
