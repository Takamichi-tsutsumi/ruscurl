extern crate getopts;

use getopts::{Options, Matches};
use std::env;
use std::process::exit;
use std::io::{Read, Write};
use std::net::{SocketAddr, ToSocketAddrs, TcpStream};


// Example usage
// ./ruscurl google.com:80
fn main() {
    let args: Vec<String> = env::args().collect();
    let opts = Options::new();

    // TODO: correctly handle args.
    let matches: Matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(_) => panic!("Args are wrong."),
    };

    // exit with return code 1 if no argument passed.
    if matches.free.len() == 0 {
        eprintln!("Too few argument. Need at least one argument 'host'. \nTry ruscurl google.com:80");
        exit(1);
    }


    let host: String = matches.free[0].clone();
    // resolve DNS
    let addrs: Vec<SocketAddr> = host.to_socket_addrs()
        .expect("Unable to resolve domain")
        .collect();

    let mut stream: TcpStream =
        TcpStream::connect(&addrs[..]).expect("Couldn't connect to server.");

    let req_str: &[u8] = &format!("GET / HTTP/1.1\n\n").into_bytes();

    let _ = stream.write_all(req_str);
    let mut bytes = [0; 2048];
    match stream.read(&mut bytes) {
        Ok(_) => {
            println!("{}", String::from_utf8_lossy(&bytes));

        }
        Err(_) => {
            panic!("Failed to read socket stream.");
        }
    }
}
