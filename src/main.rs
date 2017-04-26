#[macro_use]
extern crate error_chain;
#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;
extern crate serde_yaml;
extern crate hyper;

use hyper::server::Server;
use service::Service;

fn main() {
    let svr = Server::http("0.0.0.0:7878").unwrap();
    let _listener = svr.handle(Service);
    println!("listening...")
}
