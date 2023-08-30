use std::net::TcpListener;
use std::io::Read;
use crate::http::{Request, Handler};


pub struct Server {
    address: String,
}

impl Server {
    pub fn new(addr: String) -> Self {
        Self { address: addr }
    }

    pub fn run(self, mut handler: impl Handler) {
        println!("Server Started and Listening on {}", self.address);
        let listerner = TcpListener::bind(&self.address).unwrap();

        loop {
            match listerner.accept() {
                Ok((mut stream, address)) => {
                    println!("Connected to IP {} and Port {}", address.ip(), address.port());

                    let mut buffer = [0; 1024];
                    match stream.read(&mut buffer) {
                        Ok(_) => {
                            // println!("Received a request: {}", String::from_utf8_lossy(&buffer));
                            let response = match Request::try_from(&buffer[..]) {
                                Ok(request) => handler.handle_request(&request),
                                Err(e) => handler.handle_bad_request(&e),
                            };

                            if let Err(e) = response.send(&mut stream) {
                                println!("Failed to send response: {}", e);
                            };

                        },
                        Err(e) => println!("Failed to read from connection: {}", e),
                    }
                },
                Err(e) => {
                    println!("Failed to Establish Connection {}", e);
                },
            }
        }
    }
}