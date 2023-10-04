use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

fn main() -> std::io::Result<()> {
    // Bind the server to a specific IP address and port
    let listener = TcpListener::bind("127.0.0.1:13425")?;
    println!("Server listening on port 8080...");

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                println!("New connection: {:?}", stream.peer_addr());
                // Read the data from the client
                let mut buffer = [0; 1024];
                match stream.read(&mut buffer) {
                    Ok(n) => {
                        // Convert the received bytes to a string and print it
                        if n > 0 {
                            let received_message = String::from_utf8_lossy(&buffer[..n]);
                            println!("Received: {}", received_message);
                        }
                    }
                    Err(e) => {
                        eprintln!("Error reading from client: {}", e);
                    }
                }

                // You can send a response back to the client here if needed.
                // For now, we'll just close the connection.
                let _ = stream.shutdown(std::net::Shutdown::Both);
            }
            Err(e) => {
                eprintln!("Error accepting connection: {}", e);
            }
        }
    }

    Ok(())
}
