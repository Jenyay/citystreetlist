extern crate citystreetlist;

use citystreetlist::mosdata;

fn main () {
    match mosdata::get_version() {
        Err(e) => println!("{:?}", e),
        Ok (res) => println!("{:?}", res),
    }
}
