use thiserror::Error;

/// Types of errors that can occur when using SnowBinInfo, SnowBinWriter, and SnowBinReader.
#[derive(Error, Debug, Eq, PartialEq)]
pub enum SnowBinError {
    /// The data size used is not 8, 16, 32, or 64.
    #[error("Data Size is not 8, 16, 32, or 64.")]
    DataSizeNotAllowed,
    /// A SnowBinary file uses verify hashing, but the feature v_hash is not enabled.
    #[error("Verify hashing not enabled, please enable the \"vhash\" feature.")]
    VerifyHashingNotEnabled,
    /// The file given could not be created or opened.
    #[error("Could not create or open the file.")]
    CouldNotCreateOrOpenFile,
    /// Could not write to the file for some reason.
    #[error("Could not write to the file.")]
    IOWriteError,
    /// Header size given was < 8 bytes.
    #[error("Header must be at least 8 bytes.")]
    HeaderSizeTooSmall,
    /// The header passed in was longer than the max header length allowed.
    #[error("Header exceeds max header length.")]
    HeaderTooLong,
    /// Tried to write to file, but it was closed.
    #[error("Could not write to the file because it was closed.")]
    IOWriterClosed,
    /// Data length exceeded the max data length allowed.
    #[error("Data exceeds max length.")]
    DataTooLong,
    /// Could not read the file for some reason.
    #[error("Could not read from the file.")]
    IOReadError,
    /// File did not start with the "SNOW_BIN" header.
    #[error("File did not start with \"SNOW_BIN\" header.")]
    MalformedHeader,
    /// Tried to get a UInt from the file and failed.
    #[error("Could not pull a uint from the file when expected.")]
    MalformedUInt,
    /// Spec version of the file did not match the program Spec version.
    #[error("Spec version does not match.")]
    WrongSpecVersion,
    /// Reached the end of the file without finding the header.
    #[error("Reached the end of the file, without finding the header specified.")]
    ReachedEOF,
    /// Verify hash does not match the data extracted.
    #[error("Verification hash did not match data hash.")]
    HashDoesNotMatch,
}
