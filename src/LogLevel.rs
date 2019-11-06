use serde_repr::*;

#[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug)]
#[repr(u8)]
pub enum LogLevel {
    Fatal   = 1,
    Error   = 2,
    Warning = 4,
    Notice  = 8,
    Info    = 16,
    Debug   = 32,
    Trace   = 64,
}

impl Default for LogLevel {
    fn default() -> LogLevel {
        LogLevel::Debug
    }
}

impl LogLevel {
	pub fn to_ansi_code(&self) -> &str {
		match self {
			LogLevel::Fatal   => "\x1b[0;35m", // magenta
			LogLevel::Error   => "\x1b[0;31m", // red
			LogLevel::Warning => "\x1b[0;33m", // yellow
			LogLevel::Notice  => "\x1b[0;32m", // green
			LogLevel::Info    => "\x1b[0;34m", // blue
			LogLevel::Debug   => "\x1b[0;36m", // cyan
			LogLevel::Trace   => "\x1b[0;m",
		}
	}
	pub fn to_str(&self) -> &str {
		match self {
			LogLevel::Fatal   => "FATAL  ",
			LogLevel::Error   => "ERROR  ",
			LogLevel::Warning => "WARNING",
			LogLevel::Notice  => "NOTICE ",
			LogLevel::Info    => "INFO   ",
			LogLevel::Debug   => "DEBUG  ",
			LogLevel::Trace   => "TRACE  ",
		}
	}
}