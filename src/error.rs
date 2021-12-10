use std::{error::Error, fmt};

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum SnowBinErrorTypes {
    DataSizeNotAllowed,
    VerifyHashingNotEnabled,
    CouldNotCreateOrOpenFile,
    IOWriteError,
    HeaderSizeTooSmall,
    HeaderTooLong,
    IOWriterClosed,
    DataTooLong,
    IOReadError,
    MalformedHeader,
    MalformedUInt,
    WrongSpecVersion,
    ReachedEOF,
    HashDoesNotMatch,
}

#[derive(Debug)]
pub struct SnowBinError {
    desc: String,
    error_type: SnowBinErrorTypes,
}

impl SnowBinError {
    pub fn new(error_type: SnowBinErrorTypes) -> Self {
        let desc = match error_type {
            SnowBinErrorTypes::DataSizeNotAllowed => {
                String::from("Data Size is not 8, 16, 32, or 64.")
            }
            SnowBinErrorTypes::VerifyHashingNotEnabled => {
                String::from("Verify hashing not enabled, please enable the \"vhash\" feature.")
            }
            SnowBinErrorTypes::CouldNotCreateOrOpenFile => {
                String::from("Could not create or open the file.")
            }
            SnowBinErrorTypes::IOWriteError => String::from("Could not write to the file."),
            SnowBinErrorTypes::HeaderSizeTooSmall => {
                String::from("Header must be at least 8 bytes.")
            }
            SnowBinErrorTypes::HeaderTooLong => String::from("Header exceeds max header length."),
            SnowBinErrorTypes::IOWriterClosed => {
                String::from("Could not write to the file because it was closed.")
            }
            SnowBinErrorTypes::DataTooLong => String::from("Data exceeds max length."),
            SnowBinErrorTypes::IOReadError => String::from("Could not read from the file."),
            SnowBinErrorTypes::MalformedHeader => {
                String::from("File did not start with \"SNOW_BIN\" header.")
            }
            SnowBinErrorTypes::MalformedUInt => {
                String::from("Could not pull a uint from the file when expected.")
            }
            SnowBinErrorTypes::WrongSpecVersion => String::from("Spec version does not match."),
            SnowBinErrorTypes::ReachedEOF => {
                String::from("Reached the end of the file, without finding the header specified.")
            }
            SnowBinErrorTypes::HashDoesNotMatch => {
                String::from("Verification hash did not match data hash.")
            }
        };

        Self { desc, error_type }
    }

    pub fn error_type(&self) -> SnowBinErrorTypes {
        self.error_type
    }
}

impl fmt::Display for SnowBinError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.desc)
    }
}

impl Error for SnowBinError {
    fn description(&self) -> &str {
        &self.desc
    }
}
