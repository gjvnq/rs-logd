use serde_json::Value as JSONValue;
use serde::{Serialize, Deserialize};
use serde_json::Map;
use serde_repr::*;

#[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug)]
#[repr(u8)]
enum LogLevel {
    Fatal   = 1,
    Error   = 2,
    Warning = 4,
    Notice  = 8,
    Info    = 16,
    Debug   = 32,
    Trace   = 64,
}

#[derive(Debug, Serialize, Deserialize)]
struct LogEntry<'a> {
	/// Id of this log entry
	id: u128,
	/// Id of the sender
	sid: u128,
	/// Unix epoch of when the entry was sent
	sdt: u64,
	/// Unix epoch of when the entry was recieved
	rdt: u64,
	/// Log entry level
	lvl: LogLevel,
	/// Is this an audit log?
	iau: bool,
	/// Log entry message
	msg: &'a str,
	/// Extra information
	dat: Map<String, JSONValue>,
}
