extern crate hyper;
extern crate zip;

use std::io;
use std::io::BufReader;
use std::io::Read;
use std::io::BufRead;

use self::hyper::Client;
use self::hyper::header::Connection;

use mosdata::error;
use mosdata::defines;

#[derive(Debug)]
pub struct AreaInfo {
    pub name: String,
    pub id: u32,
    pub name_translate: String,
    pub type_name: AreaType,
    pub id_okato: u32,
    pub id_global: u32,
}

#[derive(Debug)]
pub struct StreetInfo {
    pub name: String,
    pub areas: Vec<u32>,
    pub name_short: String,
    pub name_translate: String,
    pub type_id: u32,
    pub kladr: String,
    pub id: u32,
}


#[derive(Debug)]
pub enum AreaType {
    Raion,
    Okrug,
    Poselenie,
    Unknown,
}


/// Download zip archive and return ZipArchive with it
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


/// Download and extract areas list from data.mos.ru
pub fn download_areas () -> Result<Vec<AreaInfo>, error::DownloadError> {
    let mut zip_archive = try! (get_zip_archive (defines::AREAS_URL));

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

    areas.sort_by (|a, b| a.name.cmp (&b.name));
    Ok (areas)
}


/// Move substring from end to start of the string (name)
fn from_end_to_start (name: &str, substring: &str) -> String {
    if name.ends_with(substring) {
        let right = name[..name.len() - substring.len()].trim();
        format! ("{} {}", substring, right)
    }
    else {
        name.to_string()
    }
}


fn sanitize_name (name: String) -> String {
    let mut result = from_end_to_start (&name, "район");
    result = from_end_to_start (&result, "округ");
    result = from_end_to_start (&result, "поселение");

    result
}


/// Convert area type id to AreaType
fn get_type_name (type_id: u32) -> AreaType {
    match type_id {
        2 => AreaType::Okrug,
        3 => AreaType::Raion,
        4 => AreaType::Poselenie,
        _ => AreaType::Unknown,
    }
}


/// Create AreaInfo from csv string
fn parse_area_info (line: String) -> Result<AreaInfo, error::DownloadError> {
    let items: Vec<&str> = line.split(';').collect();
    let items: Vec<&str> = items.iter().map(|item| item.trim_matches ('"')).collect();

    if items.len() != 8 {
        return Err (error::DownloadError::FormatError);
    }

    let type_name = get_type_name (try! (items[4].parse::<u32>().map_err (|_| error::DownloadError::FormatError)));

    let area_info = AreaInfo {
        name: sanitize_name (items[2].to_string()),
        id: try! (items[1].parse::<u32>().map_err (|_| error::DownloadError::FormatError)),
        name_translate: items[3].to_string(),
        type_name: type_name,
        id_okato: try! (items[5].parse::<u32>().map_err (|_| error::DownloadError::FormatError)),
        id_global: try! (items[5].parse::<u32>().map_err (|_| error::DownloadError::FormatError)),
    };

    Ok(area_info)
}
