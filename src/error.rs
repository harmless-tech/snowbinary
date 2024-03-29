/// Types of errors that can occur when using `SnowBinInfo`, `SnowBinWriter`, and `SnowBinReader`.
#[allow(clippy::module_name_repetitions)]
#[derive(Debug, Eq, PartialEq)]
pub enum SnowBinError {
    /// The data size used is not 8, 16, 32, or 64.
    DataSizeNotAllowed,
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
impl std::fmt::Display for SnowBinError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::DataSizeNotAllowed => write!(f, "Data Size is not 8, 16, 32, or 64."),
            Self::CouldNotCreateOrOpenFile => {
                write!(f, "Could not create or open the file.")
            }
            Self::IOWriteError => write!(f, "Could not write to the file."),
            Self::HeaderSizeTooSmall => write!(f, "Header must be at least 8 bytes."),
            Self::HeaderTooLong => write!(f, "Header exceeds max header length."),
            Self::IOWriterClosed => {
                write!(f, "Could not write to the file because it was closed.")
            }
            Self::DataTooLong => write!(f, "Data exceeds max length."),
            Self::IOReadError => write!(f, "Could not read from the file."),
            Self::MalformedHeader => {
                write!(f, "File did not start with \"SNOW_BIN\" header.")
            }
            Self::MalformedUInt => {
                write!(f, "Could not pull a uint from the file when expected.")
            }
            Self::WrongSpecVersion => write!(f, "Spec version does not match."),
            Self::ReachedEOF => write!(
                f,
                "Reached the end of the file, without finding the header specified."
            ),
            Self::HashDoesNotMatch => {
                write!(f, "Verification hash did not match data hash.")
            }
        }
    }
}
impl std::error::Error for SnowBinError {}
