#![allow(non_snake_case)]

use crate::log_file::*;

mod log_entry;
mod log_level;
mod log_file;

fn main() {
    println!("Hello, world!");
    let lf = new_log_file("/tmp/rs-loged.1", DEFAULT_MAX_SIZE);
    println!("{:?}", lf);
    println!("{:?}", lf.unwrap().save_header());
}
