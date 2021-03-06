extern crate getopts;
extern crate citystreetlist;

use std::env;
use std::io::stdout;
use std::io::Write;
use std::collections::HashMap;

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


fn print_streets (streets: &Vec<mosdata::StreetInfo>,
                  areas_list: &Vec<u32>,
                  areas_dict: &HashMap<u32, String>) {

    for area_id in areas_list {
        println!("");
        println! ("{}", areas_dict.get (area_id).unwrap());
        println!("");

        for street in streets {
            if street.areas.contains (area_id) {
                println! ("{}", street.name)
            }
        }
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


fn print_streets_for_areas (areas_str: String) {
    let areas_templates: Vec<String> = areas_str.split(',').map (|x| x.trim().to_string()).collect();

    print! ("Скачивание списка районов... ");
    let _ = stdout().flush();

    match mosdata::download_areas() {
        Err(e) => {
            process_error(e);
        },
        Ok (areas_list) => {
            println! ("OK");
            let areas_id_list = get_areas_id_list(&areas_templates, &areas_list);
            let areas_dict = get_areas_dict (&areas_list);

            let filter = |street_info: &mosdata::StreetInfo| {
                for street_area_id in &street_info.areas {
                    if areas_id_list.contains (&street_area_id) {
                        return true;
                    }
                }
                false
            };

            println! ("Скачивание списка улиц... ");
            let _ = stdout().flush();

            match mosdata::get_streets (filter) {
                Err(e) => {
                    process_error(e);
                },
                Ok (streets_list) => {
                    print_streets (&streets_list, &areas_id_list, &areas_dict);
                },
            };
        },
    }
}


/// Return HashMap where key is area id, value is area name
fn get_areas_dict (areas_list: &Vec<mosdata::AreaInfo>) -> HashMap<u32, String> {
    let mut hash = HashMap::new();
    for area in areas_list {
        hash.insert(area.id.clone(), area.name.clone());
    }

    hash
}


fn get_areas_id_list (areas_templates: &Vec<String>, areas_list: &Vec<mosdata::AreaInfo>) -> Vec<u32> {
    let mut result: Vec<u32> = Vec::new();
    for area in areas_list {
        for template in areas_templates {
            let tpl_lower = template.to_lowercase();
            let tpl_str = &tpl_lower[..];

            if template.starts_with ('"') && template.ends_with ('"') {
                if &area.name == &template[1..template.len() - 1] {
                    result.push (area.id);
                    break;
                }
            }
            else if area.name.to_lowercase().starts_with (tpl_str) {
                result.push (area.id);
                break;
            }
        }
    }

    result
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
    else if matches.opt_present("s") {
        match matches.opt_str("s") {
            None => print_usage(&program, opts),
            Some (areas) => print_streets_for_areas (areas),
        }
        return;
    }
    else {
        print_usage(&program, opts);
        return;
    }
}
