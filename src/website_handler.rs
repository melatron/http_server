use super::http::{Request, Response, StatusCode, Method};
use super::server::Handler;
use crate::thread_pool::ThreadPool;
use std::net::TcpStream;
use std::io::Read;
use std::fs;
use std::convert::TryFrom;

pub struct FileReader {
    public_path: String,
}

impl FileReader {
    pub fn new(public_path: String) -> Self {
        Self { public_path }
    }
    fn read_file(&self, file_path: &str) -> Option<String> {
        let path = format!("{}/{}", self.public_path, file_path);

        match fs::canonicalize(path) {
            Ok(path) => {
                if path.starts_with(&self.public_path) {
                    fs::read_to_string(path).ok()
                } else {
                    println!("Directory Traversal Attack Attempted: {}", file_path);
                    None
                }
            },
            Err(_) => None,
        }
    }
}

pub struct WebsiteHandler {
    public_path: String,
    thread_pool: ThreadPool,
}

impl WebsiteHandler {
    pub fn new(public_path: String, threads: usize) -> Self {
        Self {
            public_path,
            thread_pool: ThreadPool::new(threads)
        }
    }
}

pub fn handle_request_impl(public_path: String, mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    let file_reader = FileReader::new(public_path);
    match stream.read(&mut buffer) {
        Ok(_) => {
            println!("Received a request: {}", String::from_utf8_lossy(&buffer));

            match Request::try_from(&buffer[..]) {
                Ok(request) => {
                    let response = match request.method() {
                        Method::GET => match request.path() {
                            "/" => Response::new(StatusCode::Ok, file_reader.read_file("index.html")),
                            "/hello" => Response::new(StatusCode::Ok, file_reader.read_file("hello.html")),
                            path => match file_reader.read_file(path) {
                                Some(contents) => Response::new(StatusCode::Ok, Some(contents)),
                                None => Response::new(StatusCode::NotFound, None),
                            }
                        }
                        _ => Response::new(StatusCode::NotFound, None),
                    };

                    if let Err(e) = response.send(&mut stream) {
                        println!("Failed to send response: {}", e);
                    }
                },
                Err(e) => {
                    println!("Failed to parse request: {}", e);
                    let response = Response::new(StatusCode::BadRequest, None);
                    if let Err(e) = response.send(&mut stream) {
                        println!("Failed to send response: {}", e);
                    }
                },
            };
        },
        Err(e) => println!("Failed to read from the connection: {}", e),
    }
}

impl Handler for WebsiteHandler {
    fn handle_request(&self, stream: TcpStream) {
        let public_path = self.public_path.clone();
        self.thread_pool.execute(|| {
            handle_request_impl(public_path, stream);
        });
    }

}