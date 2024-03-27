#![warn(missing_docs)]

//! Easy to use binary file writer and reader with its own format.

mod error;
mod reader;
mod writer;

#[cfg(test)]
mod tests;

use std::{
    fs::File,
    io::{Seek, SeekFrom},
    path::PathBuf,
};

pub use crate::error::SnowBinError;

/// The version of the Spec that this library can interact with.
pub const VERSION_SPEC: u64 = 2; // Snow Binary File Format

const DEFAULT_HEADER_SIZE: u32 = 8;
const DATA_SIZES: [u8; 4] = [8, 16, 32, 64];
const DEFAULT_DATA_SIZE: usize = 3;

// In bytes.
const DATA_START: u64 = 21;
const HASH_SIZE: u32 = 32;

/// Holds information used by `SnowBinWriter` to create and write to files.
/// Default returns `SnowBinInfo` with a header size of 8 and a data size of 64.
/// # Example
/// ```
/// use snowbinary::SnowBinInfo;
///
/// let info = SnowBinInfo::default();
/// ```
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct SnowBinInfo {
    header_size: u32,
    data_size: u8,
}

impl SnowBinInfo {
    /// Creates a new `SnowBinInfo` with `header_size` and `data_size`.
    /// # Example
    /// ```
    /// use snowbinary::SnowBinInfo;
    ///
    /// let info = SnowBinInfo::new(8, 64);
    /// ```
    /// # Errors
    /// Returns `SnowBinError` if the `header_size` is < 8 or if `data_size` is not 8, 16, 32, or 64.
    pub const fn new(header_size: u32, data_size: u8) -> Result<Self, SnowBinError> {
        let header_size = if header_size >= 8 {
            header_size
        }
        else {
            return Err(SnowBinError::HeaderSizeTooSmall);
        };

        let data_size = match data_size {
            8 => 8,
            16 => 16,
            32 => 32,
            64 => 64,
            _ => return Err(SnowBinError::DataSizeNotAllowed),
        };

        Ok(Self {
            header_size,
            data_size,
        })
    }
}

impl Default for SnowBinInfo {
    fn default() -> Self {
        Self {
            header_size: DEFAULT_HEADER_SIZE,
            data_size: DATA_SIZES[DEFAULT_DATA_SIZE],
        }
    }
}

/// Allows writing to a `SnowBinary` file.
#[derive(Debug)]
pub struct SnowBinWriter {
    info: SnowBinInfo,
    file: File,
    hasher: blake3::Hasher,
    done: bool,
}

impl SnowBinWriter {
    /// Creates a new `SnowBinWriter` using the params of `SnowBinInfo`.
    /// # Example
    /// ```
    /// use std::path::PathBuf;
    ///
    /// use snowbinary::{SnowBinInfo, SnowBinWriter};
    ///
    /// let writer = SnowBinWriter::new(&SnowBinInfo::default(), PathBuf::from("file.temp"));
    /// ```
    /// # Errors
    /// Returns `SnowBinError` if the file could not be created or opened, or the file cannot be written to.
    pub fn new(info: SnowBinInfo, path: PathBuf) -> Result<Self, SnowBinError> {
        let Ok(mut file) = File::create(path)
        else {
            return Err(SnowBinError::CouldNotCreateOrOpenFile);
        };

        let mut hasher = blake3::Hasher::new();

        Self::init_file(&mut file, &mut hasher, info)?;

        Ok(Self {
            info,
            file,
            hasher,
            done: false,
        })
    }

    fn init_file(
        file: &mut File,
        hasher: &mut blake3::Hasher,
        info: SnowBinInfo,
    ) -> Result<(), SnowBinError> {
        file.rewind().map_err(|_| SnowBinError::IOWriteError)?;

        writer::write_header(file, "SNOW_BIN", 8)?;
        writer::write_u64(file, VERSION_SPEC)?;
        writer::write_u32(file, info.header_size)?;
        writer::write_u8(file, info.data_size)?;

        hasher.update(b"SNOW_BIN");
        hasher.update(&VERSION_SPEC.to_le_bytes());
        hasher.update(&info.header_size.to_le_bytes());
        hasher.update(&info.data_size.to_le_bytes());

        Ok(())
    }

    /// Writes a header and some data to a `SnowBinary` file.
    /// allowed, or the file could not be written to.
    /// # Example
    /// ```
    /// use std::path::PathBuf;
    ///
    /// use snowbinary::{error::SnowBinError, SnowBinInfo, SnowBinWriter};
    ///
    /// let mut writer = SnowBinWriter::new(&SnowBinInfo::default(), PathBuf::from("file.temp"));
    /// match &mut writer {
    ///     Ok(writer) => {
    ///         writer.write("Header", "This is data!".as_bytes()).unwrap();
    ///     }
    ///     Err(_) => {}
    /// }
    /// ```
    /// # Errors
    /// Returns `SnowBinError` if the header is too long, the data is too long, the data size is not
    //TODO Should this check for headers of the same name?
    pub fn write(&mut self, header: &str, data: &[u8]) -> Result<(), SnowBinError> {
        if !self.done {
            // Check
            if header.len() > self.info.header_size as usize {
                return Err(SnowBinError::HeaderTooLong);
            }
            let max = self.get_max_size()?;
            if data.len() as u64 > max {
                return Err(SnowBinError::DataTooLong);
            }

            // Write Data
            let header = writer::write_header(&mut self.file, header, self.info.header_size)?;
            self.hasher.update(&header);

            #[allow(clippy::cast_possible_truncation)]
            match self.info.data_size {
                8 => {
                    writer::write_u8(&mut self.file, data.len() as u8)?;
                    self.hasher.update(&(data.len() as u8).to_le_bytes());
                }
                16 => {
                    writer::write_u16(&mut self.file, data.len() as u16)?;
                    self.hasher.update(&(data.len() as u16).to_le_bytes());
                }
                32 => {
                    writer::write_u32(&mut self.file, data.len() as u32)?;
                    self.hasher.update(&(data.len() as u32).to_le_bytes());
                }
                64 => {
                    writer::write_u64(&mut self.file, data.len() as u64)?;
                    self.hasher.update(&(data.len() as u64).to_le_bytes());
                }
                _ => return Err(SnowBinError::DataSizeNotAllowed),
            }

            writer::write_bytes(&mut self.file, data)?;
            self.hasher.update(data);

            return Ok(());
        }

        Err(SnowBinError::IOWriterClosed)
    }

    fn get_max_size(&self) -> Result<u64, SnowBinError> {
        Ok(match self.info.data_size {
            8 => u64::from(u8::MAX),
            16 => u64::from(u16::MAX),
            32 => u64::from(u32::MAX),
            64 => u64::MAX,
            _ => return Err(SnowBinError::DataSizeNotAllowed),
        })
    }

    /// Closes the writer. (Alt: you could drop the writer, but this could cause a panic)
    /// # Example
    /// ```
    /// use std::path::PathBuf;
    ///
    /// use snowbinary::{error::SnowBinError, SnowBinInfo, SnowBinWriter};
    ///
    /// let mut writer = SnowBinWriter::new(&SnowBinInfo::default(), PathBuf::from("file.temp"));
    /// match &mut writer {
    ///     Ok(writer) => {
    ///         writer.write("Header", "This is data!".as_bytes()).unwrap();
    ///         writer.close().unwrap(); // Or let the writer drop.
    ///     }
    ///     Err(_) => {}
    /// }
    /// ```
    /// # Errors
    /// Returns `SnowBinError` if the file cannot be written to or the writer was already closed.
    pub fn close(&mut self) -> Result<(), SnowBinError> {
        use std::io::Write;

        if !self.done {
            let header = writer::write_header(&mut self.file, "SNOW_END", self.info.header_size)?;

            // Write hash
            self.hasher.update(&header);
            let hash = self.hasher.finalize();
            let hash = hash.as_bytes();
            writer::write_bytes(&mut self.file, hash)?;

            self.file.flush().map_err(|_| SnowBinError::IOWriteError)?;

            self.done = true;

            return Ok(());
        }

        Err(SnowBinError::IOWriterClosed)
    }
}

impl Drop for SnowBinWriter {
    fn drop(&mut self) {
        if !self.done {
            self.close()
                .expect("Could not properly drop SnowBinWriter.");
        }
    }
}

/// Allows reading from a `SnowBinary` file.
#[derive(Debug)]
pub struct SnowBinReader {
    info: SnowBinInfo,
    file: File,
}

//TODO: Allow dumping of headers.
impl SnowBinReader {
    /// Creates a new `SnowBinReader`. Params are pulled from the file info.
    /// # Example
    /// ```
    /// use std::path::PathBuf;
    ///
    /// use snowbinary::{SnowBinInfo, SnowBinReader};
    ///
    /// let reader = SnowBinReader::new(PathBuf::from("file.temp")).unwrap();
    /// ```
    /// # Errors
    /// Returns `SnowBinError` if the file could not be created or opened, or the file cannot be read from.
    pub fn new(path: PathBuf) -> Result<Self, SnowBinError> {
        let Ok(mut file) = File::open(path)
        else {
            return Err(SnowBinError::CouldNotCreateOrOpenFile);
        };

        let info = Self::read_info(&mut file)?;

        Ok(Self { info, file })
    }

    fn read_info(file: &mut File) -> Result<SnowBinInfo, SnowBinError> {
        use std::io::Read;

        // Check hash
        let hash = {
            file.rewind().map_err(|_| SnowBinError::IOReadError)?;
            let mut buffer = Vec::new();
            file.read_to_end(&mut buffer)
                .map_err(|_| SnowBinError::IOReadError)?;
            buffer.drain((buffer.len() - HASH_SIZE as usize)..buffer.len());

            blake3::hash(&buffer)
        };
        let hash = hash.as_bytes();

        {
            file.seek(SeekFrom::End(-(i64::from(HASH_SIZE))))
                .map_err(|_| SnowBinError::IOReadError)?;
            let read_hash = reader::read_bytes(file, u64::from(HASH_SIZE))?;

            if !read_hash.eq(hash) {
                return Err(SnowBinError::HashDoesNotMatch);
            }
        }

        // Read file config
        file.rewind().map_err(|_| SnowBinError::IOReadError)?;

        let snow_header = reader::read_header(file, 8)?;
        if !snow_header.eq("SNOW_BIN") {
            return Err(SnowBinError::MalformedHeader);
        }

        let version = reader::read_u64(file)?;
        if version != VERSION_SPEC {
            return Err(SnowBinError::WrongSpecVersion);
        }

        let header_size = reader::read_u32(file)?;

        let data_size = reader::read_u8(file)?;
        match data_size {
            8 | 16 | 32 | 64 => (),
            _ => return Err(SnowBinError::DataSizeNotAllowed),
        };

        Ok(SnowBinInfo {
            header_size,
            data_size,
        })
    }

    /// Reads data from the file using the header.
    /// # Example
    /// ```
    /// use std::path::PathBuf;
    ///
    /// use snowbinary::{error::SnowBinError, SnowBinInfo, SnowBinReader};
    ///
    /// let mut reader = SnowBinReader::new(PathBuf::from("file.temp"));
    /// match &mut reader {
    ///     Ok(reader) => {
    ///         let data = reader.read("Header"); // May return error
    ///     }
    ///     Err(_) => {}
    /// }
    /// ```
    /// # Errors
    /// Returns `SnowBinError` if the file cannot be read from or the end of the file was reached.
    /// # Panics
    /// Panics **could** happen if data size is over `i64::MAX`.
    pub fn read(&mut self, header: &str) -> Result<Vec<u8>, SnowBinError> {
        self.file
            .seek(SeekFrom::Start(DATA_START))
            .map_err(|_| SnowBinError::IOReadError)?;

        let mut buffer = vec![32_u8; self.info.header_size as usize];
        buffer.splice(0..header.len(), header.as_bytes().iter().copied());
        let header = String::from_utf8(buffer).map_err(|_| SnowBinError::MalformedHeader)?;

        let mut f_header = String::new();
        let mut data = Vec::new();
        let mut store = 0_u64;
        while !f_header.eq(&header) {
            f_header = reader::read_header(&mut self.file, self.info.header_size)?;

            if f_header.starts_with("SNOW_END") {
                return Err(SnowBinError::ReachedEOF);
            }

            let size = match self.info.data_size {
                8 => u64::from(reader::read_u8(&mut self.file)?),
                16 => u64::from(reader::read_u16(&mut self.file)?),
                32 => u64::from(reader::read_u32(&mut self.file)?),
                64 => reader::read_u64(&mut self.file)?,
                _ => return Err(SnowBinError::DataSizeNotAllowed),
            };

            if f_header.eq(&header) {
                data = reader::read_bytes(&mut self.file, size)?;
            }
            else {
                let mut file = &self.file;

                let tmp = if size > i64::MAX as u64 {
                    let s = size - i64::MAX as u64;
                    file.seek(SeekFrom::Current(i64::MAX))
                        .map_err(|_| SnowBinError::IOReadError)?;
                    file.seek(SeekFrom::Current(s.try_into().unwrap()))
                        .map_err(|_| SnowBinError::IOReadError)?
                }
                else {
                    file.seek(SeekFrom::Current(size.try_into().unwrap()))
                        .map_err(|_| SnowBinError::IOReadError)?
                };

                if tmp == store {
                    return Err(SnowBinError::ReachedEOF);
                }
                store = tmp;
            }
        }

        Ok(data)
    }
}
