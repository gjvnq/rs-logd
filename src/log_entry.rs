use crate::log_level::*;
use chrono::NaiveDateTime;
use serde_json::Value as JSONValue;
use serde::{Serialize, Deserialize};
use serde_json::Map;
use std::fmt::{self, Formatter, Display};

#[derive(Debug, Serialize, Deserialize)]
pub struct LogEntry {
	/// Id of the sender (SHA3-512 of the publick key OR UUID)
	pub sid: String,
	/// Unix epoch of when the entry was sent (in seconds)
	pub sts: i64,
	/// Unix epoch of when the entry was sent (nanoseconds part)
	pub stn: u32,
	/// Unix epoch of when the entry was recieved (in seconds)
	pub rts: i64,
	/// Unix epoch of when the entry was recieved (nanoseconds part)
	pub rtn: u32,
	/// Log entry level
	pub lvl: LogLevel,
	/// Is this an audit log?
	pub iau: bool,
	/// Log entry message
	pub msg: String,
	/// Extra information
	pub dat: Map<String, JSONValue>,
}

impl Default for LogEntry {
	fn default() -> LogEntry {
		let ans = LogEntry{
			sid: "00000000-0000-0000-0000-000000000000".to_string(),
			sts: 0,
			stn: 0,
			rts: 0,
			rtn: 0,
			lvl: LogLevel::Info,
			iau: false,
			msg: "".to_string(),
			dat: Map::new(),
		};
		ans
	}
}

impl LogEntry {
	/// Sent datetime
	#[allow(dead_code)]
	pub fn sdt(&self) -> NaiveDateTime {
		NaiveDateTime::from_timestamp(self.sts, self.stn)
	}

	/// Recieved datetime
	pub fn rdt(&self) -> NaiveDateTime {
		NaiveDateTime::from_timestamp(self.rts, self.rtn)
	}

	pub fn to_str(&self) -> String {
		let enc_dat = match serde_json::to_string(&self.dat) {
			Ok(v) => v,
			Err(e) => format!("{}", e),
		};
		let ans = format!("{}{} {} {} ▶ {} ◆ {}{}",
			self.lvl.to_ansi_code(),
			self.rdt().format("%Y-%m-%d %H:%M:%S%.9f"),
			self.sid,
			self.lvl.to_str(),
			self.msg,
			enc_dat,
			"\x1b[0m");
		ans
	}
}

impl Display for LogEntry {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		write!(f, "{}", self.to_str())
	}
}

mod tests {
	#[allow(unused_imports)]
	use super::*;
	#[test]
	fn test_to_str() {
		let mut e = LogEntry::default();
		e.sid = "SENDER_ID".to_string();
		assert_eq!(e.to_str(), "\u{1b}[0;34m1970-01-01 00:00:00.000000000 SENDER_ID INFO    ▶  ◆ {}\u{1b}[0m");
		assert_eq!(format!("{}", e), "\u{1b}[0;34m1970-01-01 00:00:00.000000000 SENDER_ID INFO    ▶  ◆ {}\u{1b}[0m");
	}
}