pub mod error;
mod reader;
mod writer;

use crate::error::{SnowBinError, SnowBinErrorTypes};
use std::{
    fs::File,
    io::{Seek, SeekFrom},
    path::PathBuf,
};

//TODO: Allow in buffer way.

pub const VERSION_SPEC: u64 = 1; // Snow Binary File Format

const DEFAULT_HEADER_SIZE: u64 = 8;
const DATA_SIZES: [u8; 4] = [8, 16, 32, 64];
const DEFAULT_DATA_SIZE: usize = 3;

const DATA_START: u64 = 26;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct SnowBinInfo {
    header_size: u64,
    data_size: u8,
    v_hash: bool,
}

impl SnowBinInfo {
    pub fn new(header_size: u64, data_size: u8) -> Result<Self, SnowBinError> {
        let header_size = if header_size >= 8 {
            header_size
        }
        else {
            return Err(SnowBinError::new(SnowBinErrorTypes::HeaderSizeTooSmall));
        };

        let data_size = match data_size {
            8 => 8,
            16 => 16,
            32 => 32,
            64 => 64,
            _ => return Err(SnowBinError::new(SnowBinErrorTypes::DataSizeNotAllowed)),
        };

        Ok(Self {
            header_size,
            data_size,
            v_hash: false,
        })
    }

    #[cfg(feature = "v_hash")]
    pub fn new_with_v_hash(header_size: u64, data_size: u8) -> Result<Self, SnowBinError> {
        let mut info = Self::new(header_size, data_size)?;
        info.v_hash = true;

        Ok(info)
    }

    #[cfg(feature = "v_hash")]
    pub fn default_with_v_hash() -> Self {
        Self {
            header_size: DEFAULT_HEADER_SIZE,
            data_size: DATA_SIZES[DEFAULT_DATA_SIZE],
            v_hash: true,
        }
    }
}

impl Default for SnowBinInfo {
    fn default() -> Self {
        Self {
            header_size: DEFAULT_HEADER_SIZE,
            data_size: DATA_SIZES[DEFAULT_DATA_SIZE],
            v_hash: false,
        }
    }
}

#[derive(Debug)]
pub struct SnowBinWriter {
    info: SnowBinInfo,
    file: File,
    done: bool,
}

impl SnowBinWriter {
    pub fn new(info: &SnowBinInfo, path: PathBuf) -> Result<Self, SnowBinError> {
        let mut file = match File::create(path) {
            Ok(file) => file,
            Err(_) => {
                return Err(SnowBinError::new(
                    SnowBinErrorTypes::CouldNotCreateOrOpenFile,
                ))
            }
        };

        SnowBinWriter::init_file(&mut file, info)?;

        Ok(Self {
            info: info.clone(),
            file,
            done: false,
        })
    }

    fn init_file(file: &mut File, info: &SnowBinInfo) -> Result<(), SnowBinError> {
        file.seek(SeekFrom::Start(0))
            .map_err(|_| SnowBinError::new(SnowBinErrorTypes::IOWriteError))?;

        writer::write_header(file, "SNOW_BIN", 8)?;
        writer::write_u64(file, VERSION_SPEC)?;
        writer::write_u64(file, info.header_size)?;
        writer::write_u8(file, info.data_size)?;
        writer::write_bool(file, info.v_hash)
    }

    //TODO Should this check for headers of the same name?
    pub fn write(&mut self, header: &str, data: &[u8]) -> Result<(), SnowBinError> {
        if !self.done {
            // Check
            if header.len() as u64 > self.info.header_size {
                return Err(SnowBinError::new(SnowBinErrorTypes::HeaderTooLong));
            }
            let max = self.get_max_size()?;
            if data.len() as u64 > max {
                return Err(SnowBinError::new(SnowBinErrorTypes::DataTooLong));
            }

            // Write Data
            writer::write_header(&mut self.file, header, self.info.header_size)?;
            match self.info.data_size {
                8 => writer::write_u8(&mut self.file, data.len() as u8)?,
                16 => writer::write_u16(&mut self.file, data.len() as u16)?,
                32 => writer::write_u32(&mut self.file, data.len() as u32)?,
                64 => writer::write_u64(&mut self.file, data.len() as u64)?,
                _ => return Err(SnowBinError::new(SnowBinErrorTypes::DataSizeNotAllowed)),
            }
            writer::write_bytes(&mut self.file, data)?;

            // Verify Hash
            #[cfg(feature = "v_hash")]
            if self.info.v_hash {
                use ahash::AHasher;
                use std::hash::Hasher;

                let mut hasher = AHasher::default();
                hasher.write(data);
                writer::write_u64(&mut self.file, hasher.finish())?;
            }

            return Ok(());
        }

        Err(SnowBinError::new(SnowBinErrorTypes::IOWriterClosed))
    }

    fn get_max_size(&self) -> Result<u64, SnowBinError> {
        Ok(match self.info.data_size {
            8 => u8::MAX as u64,
            16 => u16::MAX as u64,
            32 => u32::MAX as u64,
            64 => u64::MAX as u64,
            _ => return Err(SnowBinError::new(SnowBinErrorTypes::DataSizeNotAllowed)),
        })
    }

    pub fn close(&mut self) -> Result<(), SnowBinError> {
        use std::io::Write;

        if !self.done {
            writer::write_header(&mut self.file, "SNOW_END", self.info.header_size)?;
            self.file
                .flush()
                .map_err(|_| SnowBinError::new(SnowBinErrorTypes::IOWriteError))?;

            self.done = true;

            return Ok(());
        }

        Err(SnowBinError::new(SnowBinErrorTypes::IOWriterClosed))
    }
}

impl Drop for SnowBinWriter {
    fn drop(&mut self) {
        self.close().unwrap();
    }
}

#[derive(Debug)]
pub struct SnowBinReader {
    info: SnowBinInfo,
    file: File,
}

impl SnowBinReader {
    pub fn new(path: PathBuf) -> Result<Self, SnowBinError> {
        let mut file = match File::open(path) {
            Ok(file) => file,
            Err(_) => {
                return Err(SnowBinError::new(
                    SnowBinErrorTypes::CouldNotCreateOrOpenFile,
                ))
            }
        };

        let info = SnowBinReader::read_info(&mut file)?;

        Ok(Self { info, file })
    }

    fn read_info(file: &mut File) -> Result<SnowBinInfo, SnowBinError> {
        file.seek(SeekFrom::Start(0))
            .map_err(|_| SnowBinError::new(SnowBinErrorTypes::IOReadError))?;

        let snow_header = reader::read_header(file, 8)?;
        if snow_header.eq("SNOW_BIN") {
            return Err(SnowBinError::new(SnowBinErrorTypes::MalformedHeader));
        }

        let version = reader::read_u64(file)?;
        if version != VERSION_SPEC {
            return Err(SnowBinError::new(SnowBinErrorTypes::WrongSpecVersion));
        }

        let header_size = reader::read_u64(file)?;

        let data_size = reader::read_u8(file)?;
        match data_size {
            8 | 16 | 32 | 64 => (),
            _ => return Err(SnowBinError::new(SnowBinErrorTypes::DataSizeNotAllowed)),
        };

        let v_hash = reader::read_bool(file)?;
        #[cfg(not(feature = "v_hash"))]
        if v_hash {
            return Err(SnowBinError::new(SnowBinErrorTypes::VerifyHashingNotEnabled));
        }

        Ok(SnowBinInfo {
            header_size,
            data_size,
            v_hash,
        })
    }

    pub fn read(&mut self, header: &str) -> Result<Vec<u8>, SnowBinError> {
        self.file
            .seek(SeekFrom::Start(DATA_START))
            .map_err(|_| SnowBinError::new(SnowBinErrorTypes::IOReadError))?;

        let mut buffer = vec![32_u8; self.info.header_size as usize];
        buffer.splice(0..header.len(), header.as_bytes().iter().cloned());
        let header = String::from_utf8(buffer)
            .map_err(|_| SnowBinError::new(SnowBinErrorTypes::MalformedHeader))?;

        let mut f_header = String::from("");
        let mut data = Vec::new();
        while !f_header.eq(&header) {
            f_header = reader::read_header(&mut self.file, self.info.header_size)?;

            if f_header.starts_with("SNOW_END") {
                return Err(SnowBinError::new(SnowBinErrorTypes::ReachedEOF));
            }

            let size = match self.info.data_size {
                8 => reader::read_u8(&mut self.file)? as u64,
                16 => reader::read_u16(&mut self.file)? as u64,
                32 => reader::read_u32(&mut self.file)? as u64,
                64 => reader::read_u64(&mut self.file)? as u64,
                _ => return Err(SnowBinError::new(SnowBinErrorTypes::DataSizeNotAllowed)),
            };

            if f_header.eq(&header) {
                data = reader::read_bytes(&mut self.file, size)?;

                #[cfg(feature = "v_hash")]
                if self.info.v_hash {
                    use ahash::AHasher;
                    use std::hash::Hasher;

                    let f_hash = reader::read_u64(&mut self.file)?;

                    let mut hasher = AHasher::default();
                    hasher.write(&data);

                    if !f_hash.eq(&hasher.finish()) {
                        return Err(SnowBinError::new(SnowBinErrorTypes::HashDoesNotMatch));
                    }
                }
            }
            else {
                self.file
                    .seek(SeekFrom::Current(size as i64))
                    .map_err(|_| SnowBinError::new(SnowBinErrorTypes::IOReadError))?;

                #[cfg(feature = "v_hash")]
                if self.info.v_hash {
                    self.file
                        .seek(SeekFrom::Current(8))
                        .map_err(|_| SnowBinError::new(SnowBinErrorTypes::IOReadError))?;
                }
            }
        }

        Ok(data)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
