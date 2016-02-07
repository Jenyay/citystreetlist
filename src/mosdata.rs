extern crate hyper;
extern crate tempfile;

use std::io::Read;
use std::io::Write;

use self::hyper::Client;
use self::hyper::header::Connection;
use self::hyper::error;

// pub fn get_version () -> Result<String, error::Error> {
//     let client = Client::new();
//     let url = "http://api.data.mos.ru";
//
//     let mut response = try! (client.get (url).header (Connection::close()).send());
//
//     let mut body = String::new();
//     try! (response.read_to_string(&mut body));
//     Ok (body)
// }


pub fn download_areas () -> Result<String, error::Error> {
    const URL: &'static str = "http://data.mos.ru/opendata/export/2039/csv";
    let client = Client::new();

    let mut response = try! (client.get (URL).header (Connection::close()).send());
    let mut result: Vec<u8> = Vec::new();
    try! (response.read_to_end(&mut result));

    let mut fp = try! (tempfile::NamedTempFile::new());
    try! (fp.write_all (&result));

    match fp.path().to_str() {
        Some(path) => Ok (String::from (path)),
        None => Ok (String::from ("")),
    }
}
