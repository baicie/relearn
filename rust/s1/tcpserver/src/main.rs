use std::net::TcpListener;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:3000").unwrap();   
    println!("Server listening on port 3000");
    
    // let result = listener.accept().unwrap();
    for stream in listener.incoming() {
        let _stream = stream.unwrap();
        println!("Connection established!")
    }
}