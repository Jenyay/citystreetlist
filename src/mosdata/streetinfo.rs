use std::cmp;

use mosdata::error;


#[derive(Debug)]
pub struct StreetInfo {
    pub name: String,
    pub global_id: u32,
    pub areas: Vec<u32>,
    pub name_short: String,
    pub name_translate: String,
    pub type_id: u32,
    pub kladr: String,
}


impl cmp::PartialOrd for StreetInfo {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        Some (self.name.cmp (&other.name))
    }
}


impl cmp::PartialEq for StreetInfo {
    fn eq (&self, other: &Self) -> bool {
        self.type_id == other.type_id && self.name == other.name
    }
}


impl cmp::Eq for StreetInfo { }

impl cmp::Ord for StreetInfo {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        match self.partial_cmp (&other) {
            Some(ord) => ord,
            None => unreachable!(),
        }
    }
}


impl StreetInfo {
    pub fn from_raw_data (name: String,
                          name_short: String,
                          name_translate: String,
                          type_id: u32,
                          areas: String,
                          kladr: String,
                          global_id: u32) -> Result <StreetInfo, error::DownloadError> {
        let street_info = StreetInfo {
            name: name,
            global_id: global_id,
            areas: try!(StreetInfo::get_areas_list(areas)),
            name_short: name_short,
            name_translate: name_translate,
            type_id: type_id,
            kladr: kladr,
        };

        Ok(street_info)
    }


    fn get_areas_list (areas_str: String) -> Result<Vec<u32>, error::DownloadError> {
        let areas_list_str: Vec<&str> = areas_str.split(';').collect();
        let mut areas_id: Vec<u32> = Vec::new();

        for area in areas_list_str {
            let area_id = try! (area.parse::<u32>().map_err (|_| error::DownloadError::FormatError));
            areas_id.push(area_id);
        }

        Ok(areas_id)
    }
}
