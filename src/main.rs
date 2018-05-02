extern crate getopts;

mod req;

use getopts::{Options, Matches};
use std::env;
use std::process::exit;
use req::Request;


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
    let req: Request = Request::new(host);

    let res = req.send().unwrap();
    println!("Response:\n{}", String::from_utf8_lossy(&res));
}
