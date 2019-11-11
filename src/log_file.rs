use crate::log_entry::*;
use crate::log_level::*;
use serde::{Serialize, Deserialize};
use memmap::{MmapMut, MmapOptions};
use std::fs::OpenOptions;
use std::result::Result;
use std::io::Write;
use std::io;

pub const DEFAULT_START_POS: u64 = 4*1024; // 4 KiB
pub const DEFAULT_MAX_SIZE: u64 = 16*1024*1024; // 16 MiB

#[derive(Debug)]
pub struct LogFile {
	header: LogFileHeader,
	mmap: MmapMut,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LogFileHeader {
	/// Bit mask describing which log levels are saveg here
	lvls: u8,
	/// If true, this log file will store only audit entries
	only_audit: bool,
	/// Position (in bytes) where the log actually begins
	start_pos: u64,
	/// Last position (in bytes) that is written
	cur_pos: u64,
	/// Maximum size (in bytes) this file can ever be
	max_size: u64,
	/// Position (in bytes) where the time shows a discontinuity
	seam_pos: u64,
	/// Version of the file spec
	version: i16,
	/// Just a small text message to help commands like `file`
	_helper_msg: String,
}

impl Default for LogFileHeader {
    fn default() -> LogFileHeader {
        let ans = LogFileHeader{
        	lvls: log_levels_to_bitmask(&[LogLevel::Fatal, LogLevel::Error, LogLevel::Warning, LogLevel::Notice]),
        	only_audit: false,
        	start_pos: DEFAULT_START_POS,
        	cur_pos: DEFAULT_START_POS,
        	_helper_msg: "log file of github.com/gjvnq/rs-loged".to_string(),
        	max_size: DEFAULT_MAX_SIZE,
        	seam_pos: 0,
        	version: 1,
        };
        ans
    }
}

#[allow(dead_code)]
pub fn new_log_file(path: &str, max_size: u64) -> Result<LogFile, io::Error> {
	// Load the requested file and ensure we have the propper permissions
	let file = match OpenOptions::new().read(true).write(true).create(true).open(path) {
		Ok(v) => v,
		Err(e) => {println!("{}", e); return Err(e)},
	};
	// Ensure the file is non empty
	match file.set_len(max_size) {
		Err(e) => return Err(e),
		Ok(_) => {},
	};
	// Create the memory mapped structure
	let mmap = unsafe {match MmapOptions::new().map_mut(&file) {
		Err(e) => {println!("{}", e); return Err(e)},
		Ok(m) => m,
	}};
	// Finish
	return Ok(LogFile{
		header: Default::default(),
		mmap: mmap,
	})
}

impl LogFile {
	#[allow(dead_code,unused_variables)]
	pub fn save_header(&mut self) -> Result<(), io::Error> {
		match (&mut self.mmap[..]).write(b"Hello, world!") {
			Ok(_) => return Ok(()),
			Err(e) => return Err(e),
		}
	}

	#[allow(dead_code,unused_variables)]
	pub fn add_entry(&mut self, entry: &LogEntry) {
	}

	#[allow(dead_code,unused_variables)]
	pub fn sync(&mut self) {
	}

	#[allow(dead_code,unused_variables)]
	pub fn close(self) {
	}
}

// impl LogFileHeader {
// 	#[allow(dead_code,unused_variables)]
// 	pub fn add_entry(&mut self, entry: &LogEntry) {
// 	}

// 	#[allow(dead_code,unused_variables)]
// 	pub fn sync(&mut self) {
// 	}

// 	#[allow(dead_code,unused_variables)]
// 	pub fn close(self) {
// 	}
// }
