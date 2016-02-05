extern crate hyper;

use std::io::Read;

use self::hyper::Client;
use self::hyper::header::Connection;


pub fn get_version () -> String {
    let client = Client::new();
    let url = "http://api.data.mos.ru";

    let res = client.get (url)
        .header (Connection::close())
        .send();

    let mut body = String::new();

    match res {
        Result::Ok(mut response) => {
            response.read_to_string(&mut body).unwrap();
            body
        },
        Result::Err(_) => String::from ("Error"),
    }
}
