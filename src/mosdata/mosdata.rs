extern crate hyper;
extern crate zip;
extern crate csv;

use std::io;
use std::io::Read;

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

    let mut response = try! (client.get (url).header (Connection::close()).send());
    let mut buffer: Vec<u8> = Vec::new();
    try! (response.read_to_end(&mut buffer));

    // Extract file with areas
    let zip_cursor = io::Cursor::new(buffer);
    zip::ZipArchive::new (zip_cursor).map_err (error::DownloadError::Zip)
}


pub fn get_streets<F> (filter: F) -> Result<Vec<streetinfo::StreetInfo>, error::DownloadError>
                                     where F: Fn(&streetinfo::StreetInfo) -> bool {
    let mut zip_archive = try! (get_zip_archive (defines::STREETS_URL));

    // Archive must contain one file only
    assert_eq! (zip_archive.len(), 1);

    // Extract data
    let file = try! (zip_archive.by_index (0));
    let mut csv_reader = csv::Reader::from_reader(file).has_headers(true).delimiter(b';');

    let mut street_list: Vec<streetinfo::StreetInfo> = Vec::new();

    for record in csv_reader.decode() {
        match record {
            Ok (rec) => {
                let (_, _, name, name_short, name_trans, id_type, _, areas, kladr, global_id):
                    (u32, u32, String, String, String, u32, String, String, String, u32) = rec;

                let street_info = try! (streetinfo::StreetInfo::from_raw_data(name, name_short, name_trans, id_type, areas, kladr, global_id));

                if filter(&street_info) {
                    street_list.push(street_info);
                }
            },
            Err(_) => return Err (error::DownloadError::FormatError),
        };
    }

    street_list.sort_by (|a, b| a.cmp (&b));
    Ok (street_list)
}


/// Download and extract areas list from data.mos.ru
pub fn download_areas () -> Result<Vec<areainfo::AreaInfo>, error::DownloadError> {
    let mut zip_archive = try! (get_zip_archive (defines::AREAS_URL));

    // Archive must contain single file only
    assert_eq! (zip_archive.len(), 1);

    // Extract data
    let file = try! (zip_archive.by_index (0));
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
