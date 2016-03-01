extern crate getopts;
extern crate citystreetlist;

use std::env;
use std::io::stdout;
use std::io::Write;

use getopts::Options;

use citystreetlist::mosdata;
use citystreetlist::mosdata::error;

fn process_error (err: error::DownloadError) {
    match err {
        error::DownloadError::HttpError (_) => println!("Ошибка скачивания"),
        error::DownloadError::Io (e) => println!("{:?}", e),
        error::DownloadError::Zip (_) => println!("Ошибка извлечения данных из zip-архива"),
        error::DownloadError::FormatError => println!("Ошибка формата данных"),
    }
}


fn print_areas (areas: Vec<mosdata::AreaInfo>) {
    for area in areas {
        println! ("{} {}", area.type_name, area.name);
    }
}

fn download_and_print_areas () {
    print! ("Скачивание списка районов... ");
    let _ = stdout().flush();
    match mosdata::download_areas() {
        Err(e) => {
            println! ("Ошибка!");
            process_error(e);
        },
        Ok (areas) => {
            println! ("OK");
            print_areas (areas);
        },
    }
}


fn print_usage (program: &str, opts: Options) {
    let brief = format!("Использование: {} [параметры]", program);
    print!("{}", opts.usage(&brief));
}


fn main () {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    let mut opts = Options::new();
    opts.optopt ("s", "streets", "Вывести список улиц в заданных районах. Список задается через запятую", "STREETS");
    opts.optflag ("a", "areas", "Вывести список районов, округов и поселений");
    opts.optflag ("h", "help", "Вывести справку");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(_) => { print_usage(&program, opts); return; }
    };

    if matches.opt_present("h") {
        print_usage(&program, opts);
        return;
    }
    else if matches.opt_present("a") {
        download_and_print_areas();
        return;
    }
    else {
        print_usage(&program, opts);
        return;
    }
}
