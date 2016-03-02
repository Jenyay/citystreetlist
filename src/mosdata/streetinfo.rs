use std::fmt;
use std::cmp;


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
