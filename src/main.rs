extern crate citystreetlist;

use citystreetlist::mosdata;
use citystreetlist::mosdata::error;

fn process_error (err: error::DownloadError) {
    match err {
        error::DownloadError::HttpError (_) => println!("Ошибка скачивания"),
        error::DownloadError::Io (e) => println!("{:?}", e),
        error::DownloadError::Zip (_) => println!("Ошибка извлечения данных из zip-аррхива"),
        error::DownloadError::FormatError => println!("Ошибка формата данных"),
    }
}


fn print_areas (areas: Vec<mosdata::mosdata::AreaInfo>) {
    for area in areas {
        println! ("{}", area.name);
    }
}


fn main () {
    print! ("Скачивание списка районов... ");
    match mosdata::download_areas() {
        Err(e) => {
            println! ("Ошибка!");
            process_error(e)
        },
        Ok (areas) => {
            println! ("OK");
            print_areas (areas);
        },
    }
}
