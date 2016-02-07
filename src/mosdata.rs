extern crate hyper;

use std::io::Read;

use self::hyper::Client;
use self::hyper::header::Connection;
use self::hyper::error;


pub fn get_version () -> Result<String, error::Error> {
    let client = Client::new();
    let url = "http://api.data.mos.ru";

    let mut res = try! (client.get (url).header (Connection::close()).send());

    let mut body = String::new();
    try! (res.read_to_string(&mut body));
    Ok (body)
}
