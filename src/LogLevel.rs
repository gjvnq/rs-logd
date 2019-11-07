use serde_repr::*;

#[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug, Clone, Copy)]
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

pub fn log_levels_to_bitmask(lvls: &[LogLevel]) -> u8 {
	let mut ans = 0;
	for lvl in lvls.iter() {
		ans += *lvl as u8;
	}
	ans
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

mod tests {
	use super::*;
	#[test]
	fn test_log_levels_to_bitmask() {
		let b = log_levels_to_bitmask(&[]);
		assert_eq!(b, 0);
		let b = log_levels_to_bitmask(&[LogLevel::Fatal]);
		assert_eq!(b, 1);
		let b = log_levels_to_bitmask(&[LogLevel::Error]);
		assert_eq!(b, 2);
		let b = log_levels_to_bitmask(&[LogLevel::Warning]);
		assert_eq!(b, 4);
		let b = log_levels_to_bitmask(&[LogLevel::Notice]);
		assert_eq!(b, 8);
		let b = log_levels_to_bitmask(&[LogLevel::Info]);
		assert_eq!(b, 16);
		let b = log_levels_to_bitmask(&[LogLevel::Debug]);
		assert_eq!(b, 32);
		let b = log_levels_to_bitmask(&[LogLevel::Trace]);
		assert_eq!(b, 64);
		let b = log_levels_to_bitmask(&[LogLevel::Fatal, LogLevel::Error, LogLevel::Warning, LogLevel::Notice, LogLevel::Info, LogLevel::Debug, LogLevel::Trace]);
		assert_eq!(b, 127);
		let b = log_levels_to_bitmask(&[LogLevel::Fatal, LogLevel::Error, LogLevel::Warning, LogLevel::Notice]);
		assert_eq!(b, 15);
	}
}