extern crate reqwest;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

use serde::Deserialize;
use serde::json;

fn main() {
    #[derive(Deserialize, Debug)]
    struct Ip {
        origin: String,
    }

    let json: Ip = reqwest::get("http://httpbin.org/ip").json();
    println!("{:?}", json);
}
