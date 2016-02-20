extern crate citystreetlist;

use citystreetlist::mosdata;
use citystreetlist::mosdata::error;

fn process_error (err: error::DownloadError) {
    match err {
        error::DownloadError::HttpError (_) => println!("Can't download data"),
        error::DownloadError::Io (e) => println!("{:?}", e),
        error::DownloadError::Zip (_) => println!("Can't extract data from zip archive"),
        error::DownloadError::Parse (_) => println!("Integer parsing error"),
        error::DownloadError::FormatError => println!("Format Error"),
    }
}


fn print_areas (areas: Vec<mosdata::mosdata::AreaInfo>) {
    for area in areas {
        println! ("{}", area.name);
    }
}


fn main () {
    print! ("Areas downloading... ");
    match mosdata::download_areas() {
        Err(e) => {
            println! ("Fail");
            process_error(e)
        },
        Ok (areas) => {
            println! ("OK");
            print_areas (areas);
        },
    }
}
