extern crate core;

extern crate hyper;
extern crate zip;

use std::io;
use std::fmt;
use std::error;
use std::convert;
use std::result;


#[derive(Debug)]
pub enum DownloadError {
    HttpError (hyper::error::Error),
    Io (io::Error),
    Zip (zip::result::ZipError),
    FormatError,
}


pub type Result<T> = result::Result<T, DownloadError>;


impl convert::From<hyper::error::Error> for DownloadError {
    fn from (err: hyper::error::Error) -> DownloadError {
        DownloadError::HttpError (err)
    }
}


impl convert::From<io::Error> for DownloadError {
    fn from (err: io::Error) -> DownloadError {
        DownloadError::Io (err)
    }
}


impl convert::From<zip::result::ZipError> for DownloadError {
    fn from (err: zip::result::ZipError) -> DownloadError {
        DownloadError::Zip (err)
    }
}


impl fmt::Display for DownloadError {
    fn fmt (&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DownloadError::HttpError (ref err) => write! (f, "HTTP error: {}", err),
            DownloadError::Io (ref err) => write! (f, "IO error: {}", err),
            DownloadError::Zip (ref err) => write! (f, "Zip error: {}", err),
            DownloadError::FormatError => write! (f, "Format error"),
        }
    }
}


impl error::Error for DownloadError {
    fn description(&self) -> &str {
        match *self {
            DownloadError::HttpError (ref err) => err.description(),
            DownloadError::Io (ref err) => err.description(),
            DownloadError::Zip (ref err) => err.description(),
            DownloadError::FormatError => "Format error",
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            DownloadError::HttpError (ref err) => Some(err),
            DownloadError::Io (ref err) => Some(err),
            DownloadError::Zip (ref err) => Some(err),
            DownloadError::FormatError => None,
        }
    }
}
