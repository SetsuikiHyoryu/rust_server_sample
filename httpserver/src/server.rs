use crate::router::Router;
use http::httprequest::HttpRequest;
use std::{io::Read, net::TcpListener};

pub struct Server<'a> {
    socket_address: &'a str,
}

impl<'a> Server<'a> {
    pub fn new(socket_address: &'a str) -> Self {
        Server { socket_address }
    }

    pub fn run(&self) {
        let listener = TcpListener::bind(self.socket_address).unwrap();
        println!("Running on http://{}", self.socket_address);

        for stream in listener.incoming() {
            println!("Connection established!");

            let mut read_buffer = [0; 8192];

            let mut stream = stream.unwrap();
            let _ = stream.read(&mut read_buffer);

            // From & Into
            // See: https://rustwiki.org/zh-CN/rust-by-example/conversion/from_into.html
            let request: HttpRequest = String::from_utf8(read_buffer.to_vec()).unwrap().into();

            Router::route(request, &mut stream);
        }
    }
}
