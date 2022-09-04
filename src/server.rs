use crate::http::{ParseError, Request, Response, StatusCode};
use std::{io::Read, net::TcpListener};

pub trait Handler {
    fn handle_request(&mut self, request: &Request) -> Response {
        dbg!(request);
        Response::new(StatusCode::Ok, Some("<h1>Success!</h1>".to_string()))
    }

    //default implementation for any one who implements the trait but not the function
    fn handle_bad_request(&mut self, e: &ParseError) -> Response {
        println!("Failed to parse request\n{}", e);
        Response::new(StatusCode::BadRequest, Some("<h1>Fail!</h1>".to_string()))
    }
}
pub struct Server {
    address: String,
}

impl Server {
    pub fn new(address: String) -> Self {
        return Self { address };
    }

    pub fn run(&self, mut handler: impl Handler) {
        let listener = TcpListener::bind(&self.address).unwrap();
        println!("running....");
        println!("listening on {}", &self.address);

        loop {
            match listener.accept() {
                Ok((mut stream, _)) => {
                    let mut buffer = [0; 1024];
                    match stream.read(&mut buffer) {
                        Ok(_) => {
                            println!(
                                "Recieved the following request\n{}",
                                String::from_utf8_lossy(&buffer)
                            );
                            let response = match Request::try_from(&buffer[..]) {
                                Ok(request) => handler.handle_request(&request),
                                Err(e) => handler.handle_bad_request(&e),
                            };

                            if let Err(e) = response.send(&mut stream) {
                                dbg!("Failed to send respone");
                                println!("{:?}", e)
                            }
                        }
                        Err(e) => {
                            println!("Failed to read from connection\n {}", e)
                        }
                    };
                }
                Err(e) => {
                    println!("failed to connect because of {}", e)
                }
            }
        }
    }
}

impl Handler for Server {}
