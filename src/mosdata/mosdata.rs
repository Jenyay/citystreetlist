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

#[derive(Debug)]
pub struct AreaInfo {
    name: String,
    id: u32,
    name_translate: String,
    id_type: u32,
    id_okato: u32,
    id_global: u32,
}


fn get_zip_archive<'a> (url: &str) -> Result<zip::read::ZipArchive<io::Cursor<Vec<u8>>>, error::DownloadError> {
    // Download zip file with areas
    let client = Client::new();

    let mut response = try! (client.get (url).header (Connection::close()).send().map_err (error::DownloadError::HttpError));
    let mut buffer: Vec<u8> = Vec::new();
    try! (response.read_to_end(&mut buffer).map_err (error::DownloadError::Io));

    // Extract file with areas
    let zip_cursor = io::Cursor::new(buffer);
    zip::ZipArchive::new (zip_cursor).map_err (error::DownloadError::Zip)
}


pub fn download_areas () -> Result<Vec<AreaInfo>, error::DownloadError> {
    const URL: &'static str = "http://data.mos.ru/opendata/export/2039/csv";
    let mut zip_archive = try! (get_zip_archive (URL));

    // Archive must contain single file only
    assert_eq! (zip_archive.len(), 1);

    // Extract data
    let file = try! (zip_archive.by_index (0).map_err (error::DownloadError::Zip));
    let mut file_buffer = BufReader::new (file);

    let mut areas: Vec<AreaInfo> = Vec::new();
    let mut first = true;

    loop {
        let mut area_str = String::new();
        match file_buffer.read_line(&mut area_str) {
            Err(_) => {
                println! ("Error!");
                break
            },
            Ok (0) => break,
            Ok (_) => {
                if first {
                    first = false;
                }
                else {
                    let area_info = try! (parse_area_info (area_str));
                    areas.push(area_info);
                }
            },
        }
    }
    Ok (areas)
}


fn parse_area_info (line: String) -> Result<AreaInfo, error::DownloadError> {
    let items: Vec<&str> = line.split(';').collect();
    let items: Vec<&str> = items.iter().map(|item| item.trim_matches ('"')).collect();
    assert_eq! (items.len(), 8);

    let area_info = AreaInfo {
        name: items[2].to_string(),
        id: try! (items[1].parse::<u32>().map_err (error::DownloadError::Parse)),
        name_translate: items[3].to_string(),
        id_type: try! (items[4].parse::<u32>().map_err (error::DownloadError::Parse)),
        id_okato: try! (items[5].parse::<u32>().map_err (error::DownloadError::Parse)),
        id_global: try! (items[5].parse::<u32>().map_err (error::DownloadError::Parse)),
    };

    Ok(area_info)
}
