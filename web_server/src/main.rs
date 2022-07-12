use std::net::{TcpListener,TcpStream}; 
//get access to certain traits that let us read from and write to the stream 
use std::io::prelude::*; 
use std::fs; 
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

    println!("Read: {}",String::from_utf8_lossy(&buffer[..])); 
    let status_line = "HTTP/1.1 200 OK\r\n\r\n"; 
    let contents = fs::read_to_string("hello.html").unwrap(); 
    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}", 
        status_line, 
        contents.len(), 
        contents, 
    );
    println!("Response: {}",response); 
    //error handling here 
    stream.write(response.as_bytes()).unwrap(); 
    stream.flush().unwrap(); 
}