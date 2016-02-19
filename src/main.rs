extern crate citystreetlist;

use citystreetlist::mosdata;
use citystreetlist::mosdata::error;

fn process_error (err: error::DownloadError) {
    match err {
        error::DownloadError::HttpError (_) => println!("Can't download data"),
        error::DownloadError::Io (e) => println!("{:?}", e),
        error::DownloadError::Zip (_) => println!("Can't extract data from zip archive"),
        error::DownloadError::Parse (_) => println!("Integer parsing error"),
    }
}

fn main () {
    match mosdata::download_areas() {
        Err(e) => process_error(e),
        Ok (res) => {},
    }
}
