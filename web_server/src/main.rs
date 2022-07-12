use std::net::{TcpListener,TcpStream}; 
//get access to certain traits that let us read from and write to the stream 
use std::io::prelude::*; 
fn main() {
    //bind a listener to the local host 
    //handle error case is important 
    let listener = 
    TcpListener::bind("127.0.0.1:7878").unwrap(); 

    for stream in listener.incoming() {
        let stream = stream.unwrap(); 
        handle_connection(stream); 
    }
}


//read data from the tcp stream and print it out
//the stream is mut cause the internal state of the stream might change 
fn handle_connection(mut stream:TcpStream){
    let mut buffer = [0;1024]; 

    stream.read(&mut buffer).unwrap();

    println!("Request: {}", String::from_utf8_lossy(&buffer[..])); 
}