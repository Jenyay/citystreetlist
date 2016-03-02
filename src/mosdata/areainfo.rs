use std::fmt;
use std::cmp;


#[derive(Debug)]
pub struct AreaInfo {
    pub name: String,
    pub id: u32,
    pub name_translate: String,
    pub type_name: AreaType,
    pub id_okato: u32,
    pub id_global: u32,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum AreaType {
    Unknown = 1,
    Raion,
    Okrug,
    Poselenie,
}


impl AreaType {
    /// Convert area type id to AreaType
    pub fn from_id (type_id: u32) -> AreaType {
        match type_id {
            2 => AreaType::Okrug,
            3 => AreaType::Raion,
            4 => AreaType::Poselenie,
            _ => AreaType::Unknown,
        }
    }
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


impl AreaInfo {
    pub fn from_raw_data (area_id: u32,
                          name: String,
                          name_translate: String,
                          type_id: u32,
                          id_okato: u32,
                          id_global: u32) -> AreaInfo {
        let type_name = AreaType::from_id (type_id);

        AreaInfo {
            name: AreaInfo::sanitize_area_name (name),
            id: area_id,
            name_translate: name_translate,
            type_name: type_name,
            id_okato: id_okato,
            id_global: id_global,
        }
    }


    fn sanitize_area_name (name: String) -> String {
        AreaInfo::remove_type_name (&name)
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
}
