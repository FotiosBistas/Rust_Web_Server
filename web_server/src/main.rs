use std::net::TcpListener; 

fn main() {
    //bind a listener to the local host 
    //handle error case is important 
    let listener = 
    TcpListener::bind("127.0.0.1:7878").unwrap(); 

    for stream in listener.incoming() {
        let stream = stream.unwrap(); 
        println!("Connection established");
    }
}
