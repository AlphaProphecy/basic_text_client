extern crate tokio;
use std::path::Path;
use tokio::prelude::Future;


pub fn send(path_string: String, ip_string: String){
    
    //println!("{}",ip_string);
    //println!("IP to connect to: {}", ip_string);

    let path = Path::new(&path_string);

    let task = tokio::fs::File::open(path_string).and_then(|file| {
        // do something with the file ...
        file.metadata().map(|md| println!("{:?}", md))
    }).map_err(|e| {
        // handle errors
        eprintln!("IO error: {:?}", e);
    });


    tokio::run(task);
    path_string.to_lowercase();
}