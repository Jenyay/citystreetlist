extern crate citystreetlist;

use citystreetlist::mosdata;

fn main () {
    match mosdata::download_areas() {
        Err(e) => println!("{:?}", e),
        Ok (res) => println!("{:?}", res),
    }
}
