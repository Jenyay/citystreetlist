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
