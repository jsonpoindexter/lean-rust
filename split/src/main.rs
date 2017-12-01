extern crate hyper;
use std::collections::HashMap;
#[macro_use]
extern crate maplit;

use hyper::header::{Cookie, Header};

fn main() {

    let credentials: HashMap<String, String> = hashmap! {
        "cookies".into() => "sessionid=945315306a1c5d16db8a2dcb; \
        steamCountry=US%7Cae6f3026e35a6ac0236b9adfbca4d4ca; \
        timezoneOffset=-28800,0; \
        steamLogin=76561198217382265%7C%7C74A3C37AC75B0A94F826CE7CD8354FD3D5C1B686".into()
    };
    let cookies = credentials["cookies"].to_owned();
    let cookies = Cookie::parse_header(&cookies.into()).unwrap();
    let user_id = cookies.get("steamLogin").unwrap().split("%").into_iter().next().unwrap().to_owned();
    println!("{:?}", user_id);
}
