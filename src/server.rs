use std::net::TcpListener;
use std::io::{Read, Write};
use crate::http::{Request, Response, StatusCode};


pub struct Server {
    address: String,
}

impl Server {
    pub fn new(addr: String) -> Self {
        Self { address: addr }
    }

    pub fn run(self) {
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
                            match Request::try_from(&buffer[..]) {
                                Ok(request) => {
                                    dbg!(request);
                                    let response = Response::new(StatusCode::Ok, Some("<h1>Hello World!</h1>".to_string()));
                                    write!(stream, "{}", response).unwrap();
                                },
                                Err(e) => println!("Failed to Parse the Request: {}", e),
                            }

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