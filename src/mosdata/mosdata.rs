extern crate hyper;
extern crate tempfile;
extern crate zip;

use std::io;
use std::io::BufReader;
use std::io::Read;
use std::io::BufRead;

use self::hyper::Client;
use self::hyper::header::Connection;

use mosdata::error;


pub fn download_areas () -> Result<String, error::DownloadError> {
    const URL: &'static str = "http://data.mos.ru/opendata/export/2039/csv";

    // Download zip file with areas
    let client = Client::new();

    let mut response = try! (client.get (URL).header (Connection::close()).send().map_err (error::DownloadError::HttpError));
    let mut buffer: Vec<u8> = Vec::new();
    try! (response.read_to_end(&mut buffer).map_err (error::DownloadError::Io));

    // Extract file with areas
    let zip_cursor = io::Cursor::new(buffer);
    let mut zip = try! (zip::ZipArchive::new (zip_cursor).map_err (error::DownloadError::Zip));

    // Archive must contain single file only
    assert_eq! (zip.len(), 1);

    // Extract data
    let file = try! (zip.by_index (0).map_err (error::DownloadError::Zip));
    let mut file_buffer = BufReader::new (file);

    loop {
        let mut area = String::new();
        match file_buffer.read_line(&mut area) {
            Ok (0) | Err(_) => break,
            Ok (_) => println! ("{}", area.trim()),
        }
    }

    Ok ("areas".to_string())
}
