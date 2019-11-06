use crate::LogLevel::LogLevel;
use chrono::NaiveDateTime;
use serde_json::Value as JSONValue;
use serde::{Serialize, Deserialize};
use serde_json::Map;
use std::fmt::{self, Formatter, Display};

#[derive(Debug, Serialize, Deserialize, Default)]
struct LogEntry<'a> {
	/// Id of this log entry
	id: u128,
	/// Id of the sender (SHA3-512 of the publick key)
	sid: &'a str,
	/// Unix epoch of when the entry was sent (in seconds)
	sts: i64,
	/// Unix epoch of when the entry was sent (nanoseconds part)
	stn: u32,
	/// Unix epoch of when the entry was recieved (in seconds)
	rts: i64,
	/// Unix epoch of when the entry was recieved (nanoseconds part)
	rtn: u32,
	/// Log entry level
	lvl: LogLevel,
	/// Is this an audit log?
	iau: bool,
	/// Log entry message
	msg: &'a str,
	/// Extra information
	dat: Map<String, JSONValue>,
}

impl LogEntry<'_> {
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

impl Display for LogEntry<'_> {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_str())
    }
}

mod tests {
	use super::*;
	#[test]
	fn test_to_str() {
		let mut e = LogEntry::default();
		e.sid = "SENDER_ID";
	    assert_eq!(e.to_str(), "\u{1b}[0;36m1970-01-01 00:00:00.000000000 SENDER_ID DEBUG   ▶  ◆ {}\u{1b}[0m");
	    assert_eq!(format!("{}", e), "\u{1b}[0;36m1970-01-01 00:00:00.000000000 SENDER_ID DEBUG   ▶  ◆ {}\u{1b}[0m");
	}
}