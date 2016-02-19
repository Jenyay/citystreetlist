extern crate core;

extern crate hyper;
extern crate zip;

use std::io;
use self::core::num;


#[derive(Debug)]
pub enum DownloadError {
    HttpError (hyper::error::Error),
    Io (io::Error),
    Zip (zip::result::ZipError),
    Parse (core::num::ParseIntError),
}
