extern crate hyper;
extern crate zip;
extern crate csv;

use std::io;
use std::io::BufReader;
use std::io::Read;
use std::io::BufRead;

use self::hyper::Client;
use self::hyper::header::Connection;

use mosdata::error;
use mosdata::defines;
use mosdata::areainfo;
use mosdata::streetinfo;


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


pub fn get_streets (filter: fn(&streetinfo::StreetInfo) -> bool) -> Result<Vec<streetinfo::StreetInfo>, error::DownloadError> {
    let mut zip_archive = try! (get_zip_archive (defines::STREETS_URL));

    // Extract data
    let file = try! (zip_archive.by_index (0).map_err (error::DownloadError::Zip));
    let mut file_buffer = BufReader::new (file);

    let mut streets_list: Vec<streetinfo::StreetInfo> = Vec::new();
    let mut first = true;

    loop {
        let mut street_str = String::new();
        match file_buffer.read_line(&mut street_str) {
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
                    let street_info = try! (parse_street_info (street_str));
                    if filter (&street_info) {
                        streets_list.push(street_info);
                    }
                }
            },
        }
    }

    streets_list.sort_by (|a, b| a.cmp (&b));
    Ok (streets_list)
}


/// Download and extract areas list from data.mos.ru
pub fn download_areas () -> Result<Vec<areainfo::AreaInfo>, error::DownloadError> {
    let mut zip_archive = try! (get_zip_archive (defines::AREAS_URL));

    // Archive must contain single file only
    assert_eq! (zip_archive.len(), 1);

    // Extract data
    let file = try! (zip_archive.by_index (0).map_err (error::DownloadError::Zip));
    let mut csv_reader = csv::Reader::from_reader(file).has_headers(true).delimiter(b';');

    let mut areas: Vec<areainfo::AreaInfo> = Vec::new();

    for record in csv_reader.decode() {
        match record {
            Ok (rec) => {
                let (_, area_id, name, name_translate, type_id, id_okato, id_global):
                    (u32, u32, String, String, u32, u32, u32) = rec;

                areas.push(areainfo::AreaInfo::from_raw_data(area_id, name, name_translate, type_id, id_okato, id_global));
            },
            Err(_) => return Err (error::DownloadError::FormatError),
        };
    }

    areas.sort_by (|a, b| a.cmp (&b));
    Ok (areas)
}


fn sanitize_street_name (name: String) -> String {
    name
}


/// Create StreetInfo from csv string
fn parse_street_info (line: String) -> Result<streetinfo::StreetInfo, error::DownloadError> {
    let items: Vec<&str> = line.split(';').collect();
    let items: Vec<&str> = items.iter().map(|item| item.trim_matches ('"')).collect();

    if items.len() != 11 {
        return Err (error::DownloadError::FormatError);
    }

    // let streetInfo = StreetInfo {
    //     name: sanitize_street_name (items[2].to_string()),
    //     areas: try! (items[7].split(';')...)
    // }
    unimplemented!();
}
