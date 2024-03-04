use super::router::Router;
use http::httprequest::HttpRequest;
use std::io::prelude::*;
use std::net::TcpListener;
use std::str;

pub struct Server<'a> {
    socker_addr: &'a str,
}
impl<'a> Server<'a>{
    pub fn new(socket_addr: &'a str) -> Self {
        Server { socker_addr }
    }
    pub fn run(&self){
    //start a server listening on socket address
    let connection_listener = TcpListener::bind(self.socket_addr).unwrap();
    println!("Running on {}", self.socket_addr);
    //Listen to incoming connections in a loop 
    for stream in connection_listener.incoming() {
        let mut stream = stream.unwrap();
        println!("Connection established");
        let mut read_buffer = [0;90];
        stream.read(&mut read_buffer).unwrap();
        //convert http request tp rust data structure
        let req: HttpRequest = String::from_utf8(read_buffer.to_vec()).unwrap().into();
    //Route request to appropriate handler
        Router::route(req, &mut stream);
        }
    }
    }
