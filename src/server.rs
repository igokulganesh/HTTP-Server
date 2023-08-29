use std::net::TcpListener;
use std::io::{Read, Write};
use crate::http::{Request, Response, StatusCode, response};


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
                            let response = match Request::try_from(&buffer[..]) {
                                Ok(request) => {
                                    dbg!(request);
                                    Response::new(StatusCode::Ok, Some("<h1>Hello World! </h1>".to_string()))
                                },
                                Err(e) => {
                                    println!("Failed to Parse the Request: {}", e);
                                    Response::new(StatusCode::BadRequest, None)
                                },
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