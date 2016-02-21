extern crate hyper;
extern crate zip;

use std::io;
use std::io::BufReader;
use std::io::Read;
use std::io::BufRead;
use std::fmt;
use std::cmp;

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


#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum AreaType {
    Unknown = 1,
    Raion,
    Okrug,
    Poselenie,
}


impl fmt::Display for AreaType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &AreaType::Raion => write! (f, "район"),
            &AreaType::Okrug => write! (f, "округ"),
            &AreaType::Poselenie => write! (f, "поселение"),
            &AreaType::Unknown => write! (f, ""),
        }
    }
}


impl cmp::PartialOrd for AreaInfo {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        if self.type_name == other.type_name {
            Some (self.name.cmp (&other.name))
        }
        else {
            Some (self.type_name.cmp (&other.type_name))
        }
    }
}


impl cmp::PartialEq for AreaInfo {
    fn eq (&self, other: &Self) -> bool {
        self.type_name == other.type_name && self.name == other.name
    }
}

impl cmp::Eq for AreaInfo { }

impl cmp::Ord for AreaInfo {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        match self.partial_cmp (&other) {
            Some(ord) => ord,
            None => unreachable!(),
        }
    }
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

    areas.sort_by (|a, b| a.cmp (&b));
    Ok (areas)
}


fn remove_type_name (name: &str) -> String {
    let type_names = vec! ["район", "округ", "поселение"];
    let mut result = name.trim();

    for type_name in type_names {
        if result.ends_with (type_name) {
            result = result[..result.len() - type_name.len()].trim();
        }

        if result.starts_with (type_name) {
            result = result[type_name.len()..].trim();
        }
    }

    result.to_string()
}


fn sanitize_name (name: String) -> String {
    remove_type_name (&name)
}


/// Convert area type id to AreaType
fn get_type_area (type_id: u32) -> AreaType {
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

    let type_name = get_type_area (try! (items[4].parse::<u32>().map_err (|_| error::DownloadError::FormatError)));

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
