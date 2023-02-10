use std::{error::Error, fmt};

/// Types of errors that can occur when using SnowBinInfo, SnowBinWriter, and SnowBinReader.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum SnowBinErrorTypes {
    /// The data size used is not 8, 16, 32, or 64.
    DataSizeNotAllowed,
    /// A SnowBinary file uses verify hashing, but the feature v_hash is not enabled.
    VerifyHashingNotEnabled,
    /// The file given could not be created or opened.
    CouldNotCreateOrOpenFile,
    /// Could not write to the file for some reason.
    IOWriteError,
    /// Header size given was < 8 bytes.
    HeaderSizeTooSmall,
    /// The header passed in was longer than the max header length allowed.
    HeaderTooLong,
    /// Tried to write to file, but it was closed.
    IOWriterClosed,
    /// Data length exceeded the max data length allowed.
    DataTooLong,
    /// Could not read the file for some reason.
    IOReadError,
    /// File did not start with the "SNOW_BIN" header.
    MalformedHeader,
    /// Tried to get a UInt from the file and failed.
    MalformedUInt,
    /// Spec version of the file did not match the program Spec version.
    WrongSpecVersion,
    /// Reached the end of the file without finding the header.
    ReachedEOF,
    /// Verify hash does not match the data extracted.
    HashDoesNotMatch,
}

/// Holds a description of the error and the type of error it was.
#[derive(Debug)]
pub struct SnowBinError {
    desc: String,
    error_type: SnowBinErrorTypes,
}

impl SnowBinError {
    /// Creates a new SnowBinError using this type and assigns it a description.
    /// # Example
    /// ```
    /// use snowbinary::error::{SnowBinError, SnowBinErrorTypes};
    ///
    /// let err = SnowBinError::new(SnowBinErrorTypes::ReachedEOF);
    /// err.error_type(); // == SnowBinErrorTypes::ReachedEOF
    /// eprintln!("{}", err); // == Reached the end of the file, without finding the header specified.
    /// ```
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

    /// Returns the type of error this is.
    /// # Example
    /// ```
    /// use snowbinary::error::{SnowBinError, SnowBinErrorTypes};
    ///
    /// let err = SnowBinError::new(SnowBinErrorTypes::ReachedEOF);
    /// err.error_type(); // == SnowBinErrorTypes::ReachedEOF
    /// ```
    pub fn error_type(&self) -> SnowBinErrorTypes {
        self.error_type
    }
}

impl Error for SnowBinError {}

impl fmt::Display for SnowBinError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.desc)
    }
}
