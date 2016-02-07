extern crate hyper;
extern crate tempfile;
extern crate zip;

use std::io;
use std::io::Read;

use self::hyper::Client;
use self::hyper::header::Connection;


pub fn download_areas () -> Result<String, String> {
    const URL: &'static str = "http://data.mos.ru/opendata/export/2039/csv";

    // Download zip file with areas
    let client = Client::new();

    let mut response = try! (client.get (URL).header (Connection::close()).send().map_err (|e| e.to_string()));
    let mut buffer: Vec<u8> = Vec::new();
    try! (response.read_to_end(&mut buffer).map_err (|e| e.to_string()));

    // Extract file with areas
    let zip_cursor = io::Cursor::new(buffer);
    let mut zip = try! (zip::ZipArchive::new (zip_cursor).map_err (|e| e.to_string()));

    // Archive must contain single file only
    assert_eq! (zip.len(), 1);

    // Extract data
    let mut areas = String::new();
    let mut file = try! (zip.by_index (0).map_err (|e| e.to_string()));
    try! (file.read_to_string (&mut areas).map_err (|e| e.to_string()));

    Ok (areas)
}
