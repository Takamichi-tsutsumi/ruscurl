use std::io::Write;
use std::io::prelude::*;
use std::net::TcpStream;


fn main() {
    let mut stream = TcpStream::connect("216.58.197.4:80").expect("couldn't connect");
    let _ = stream.write_all(b"GET / HTTP/1.1 \nHost: google.com\n\n");
    let mut bytes = [0; 1024];
    let res = stream.read(&mut bytes);
    match res {
        Ok(s) => {
            println!("Read {} bytes", s);
            println!("{}", String::from_utf8_lossy(&bytes));
        }
        Err(_e) => println!("Fail to read"),
    }
}
