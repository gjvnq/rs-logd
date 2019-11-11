#![allow(non_snake_case,unused_imports)]

use crate::log_file::*;
use crate::log_level::*;
use crate::log_file::*;
use crate::log_error::*;

mod log_entry;
mod log_level;
mod log_file;
mod log_error;

fn main() {
	println!("Hello, world!");
	let lf = new_log_file("/tmp/rs-loged.1", DEFAULT_MAX_SIZE);
	println!("{:?}", lf);
	println!("{:?}", lf.unwrap().save_header());
}
