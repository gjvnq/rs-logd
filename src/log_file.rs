use crate::log_error::*;
use crate::log_entry::*;
use crate::log_level::*;
use serde::{Serialize, Deserialize};
use memmap::{MmapMut, MmapOptions};
use rmp_serde::Serializer;
use std::fs::OpenOptions;
use std::result::Result;
use tempfile::tempdir;
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
	pub lvls: u8,
	/// If true, this log file will store only audit entries
	pub only_audit: bool,
	/// Position (in bytes) where the log actually begins
	pub start_pos: u64,
	/// Last position (in bytes) that is written
	pub cur_pos: u64,
	/// Maximum size (in bytes) this file can ever be
	pub max_size: u64,
	/// Position (in bytes) where the time shows a discontinuity
	pub seam_pos: u64,
	/// Version of the file spec
	pub version: i16,
	/// Just a small text message to help commands like `file`
	pub _helper_msg: String,
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
	file.set_len(max_size)?;
	// Create the memory mapped structure
	let mmap = unsafe {MmapOptions::new().map_mut(&file)?};
	// Finish
	return Ok(LogFile{
		header: Default::default(),
		mmap: mmap,
	})
}

impl LogFile {
	pub fn save_header(&mut self) -> Result<(), Error> {
		let dat = self.header.as_vec_u8()?;
		(&mut self.mmap[..]).write(&dat)?;
		return Ok(());
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

impl LogFileHeader {
	pub fn as_vec_u8(&self) -> Result<Vec<u8>, rmp_serde::encode::Error> {
		let mut buf = Vec::new();
		self.serialize(&mut Serializer::new(&mut buf))?;
		return Ok(buf)
	}
}

mod tests {
	#[allow(unused_imports)]
	use super::*;

	#[test]
	fn test_new_log_file_1() {
		new_log_file("/ttmp/rs-loged.1", DEFAULT_MAX_SIZE).expect_err("This shouldn't work beacuse the folder shouldn't exist");
	}

	#[test]
	fn test_new_log_file_2() {
		let dir = tempdir().unwrap();
		let file_path = dir.path().join("rs-loged");
		let lf = new_log_file(file_path.to_str().unwrap(), DEFAULT_MAX_SIZE).expect("The file should have been created automatically");
		drop(lf);
		dir.close().expect("");
	}

	#[test]
	fn test_as_vec_u8() {
		let h: LogFileHeader = Default::default();
		let b = h.as_vec_u8().unwrap();
		assert_eq!(b, vec![152, 15, 194, 205, 16, 0, 205, 16, 0, 206, 1, 0, 0, 0, 0, 1, 217, 37, 108, 111, 103, 32, 102, 105, 108, 101, 32, 111, 102, 32, 103, 105, 116, 104, 117, 98, 46, 99, 111, 109, 47, 103, 106, 118, 110, 113, 47, 114, 115, 45, 108, 111, 103, 101, 100]);
	}
}
