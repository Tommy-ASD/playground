use std::{
    io::{BufReader, Write},
    net::TcpListener,
};

fn main() {
    let listener: TcpListener = TcpListener::bind("127.0.0.1:13425").unwrap();
    for stream in listener.incoming() {
        if let Ok(mut stream) = stream {
            let reader = BufReader::new(&stream);

            println!("Recieved data: {reader:?}");

            let message = "Go back to your terminal :)";
            let response = format!(
                "HTTP/1.1 200 OK\r\ncontent-length: {}\r\n\r\n{}",
                message.len(),
                message
            );
            stream.write_all(response.as_bytes()).unwrap();
            // The server will terminate itself after revoking the token.
        }
    }
}
