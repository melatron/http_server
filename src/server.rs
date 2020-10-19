use std::net::{TcpStream, TcpListener};



pub trait Handler {
    fn handle_request(&self, stream: TcpStream);
}

pub struct Server {
    addr: String,
}

impl Server {
    pub fn new(addr: String) -> Self {
        Self { addr }
    }

    pub fn run(self, handler: impl Handler) {
        println!("Listening on {}", self.addr);

        let listener = TcpListener::bind(&self.addr).unwrap();

        loop {
            match listener.accept() {
                Ok((stream, _)) => {
                    handler.handle_request(stream)
                },
                Err(e) => println!("Failed to establish a connection: {}", e),
            }
        }
    }
}
