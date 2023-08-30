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
                Ok((mut stream, _address)) => {
                    let mut buffer = [0; 1024];
                    match stream.read(&mut buffer) {
                        Ok(_) => {
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