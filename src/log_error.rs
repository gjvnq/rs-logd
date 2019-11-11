#[derive(Debug)]
#[allow(dead_code)]
pub enum Error {
	IOErr(std::io::Error),
	MsgPackEncode(rmp_serde::encode::Error),
	MsgPackDecode(rmp_serde::decode::Error),
}

impl From<std::io::Error> for Error {
	fn from(e: std::io::Error) -> Self {
		Error::IOErr(e)
	}
}

impl From<rmp_serde::encode::Error> for Error {
	fn from(e: rmp_serde::encode::Error) -> Self {
		Error::MsgPackEncode(e)
	}
}

impl From<rmp_serde::decode::Error> for Error {
	fn from(e: rmp_serde::decode::Error) -> Self {
		Error::MsgPackDecode(e)
	}
}