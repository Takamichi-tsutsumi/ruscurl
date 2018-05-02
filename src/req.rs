use std::io::{Read, Write};
use std::net::{SocketAddr, ToSocketAddrs, TcpStream};

pub struct Request {
    host: String,
    addrs: Vec<SocketAddr>,
}


impl Request {
    pub fn new(host: String) -> Request {
        let addrs: Vec<SocketAddr> = host.to_socket_addrs()
            .expect("Unable to resolve domain")
            .collect();

        Request { host, addrs }
    }

    fn connect(&self) -> TcpStream {
        TcpStream::connect(&self.addrs[..]).expect("couldn't connect to the server")
    }

    pub fn send(&self) -> Option<Vec<u8>> {
        let mut stream = self.connect();
        let req_str = format!("GET / HTTP/1.1\nHost: {}\n\n", self.host);
        println!("Request:\n{}", req_str);
        let _ = stream.write_all(&req_str.into_bytes());

        let mut bytes = [0; 2048];
        let res = match stream.read(&mut bytes) {
            Ok(_) => bytes,
            Err(_) => panic!("Failed to read socket stream."),
        };

        Some(res.to_vec())
    }
}
