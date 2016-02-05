extern crate citystreetlist;

use citystreetlist::mosdata;

fn main () {
    let version = mosdata::get_version();
    println! ("{}", version);
}
