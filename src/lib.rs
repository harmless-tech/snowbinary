use crate::error::{SnowBinError, SnowBinErrorTypes};
use std::{fs::File, path::PathBuf};
use std::io::{Seek, SeekFrom};

pub mod error;
mod writer;

const VERSION: u64 = 0; // Snow Binary File Format

const MIN_VERSION: u64 = 0;
const DEFAULT_HEADER_SIZE: u64 = 8;
const DATA_SIZES: [u8; 4 /*5*/] = [8, 16, 32, 64/*, 128*/];
const DEFAULT_DATA_SIZE: usize = 3;

#[cfg(feature = "v_hash")]
const VERIFY_HASHING: bool = true;
#[cfg(not(feature = "v_hash"))]
const VERIFY_HASHING: bool = false;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct SnowBinInfo {
    header_size: u64,
    data_size: u8,
    v_hash: bool,
}
impl SnowBinInfo {
    pub fn new() -> Self {
        Self {
            header_size: DEFAULT_HEADER_SIZE,
            data_size: DATA_SIZES[DEFAULT_DATA_SIZE],
            v_hash: false,
        }
    }

    pub fn new_with_v_hash() -> Result<Self, SnowBinError> {
        if !VERIFY_HASHING {
            return Err(SnowBinError::new(SnowBinErrorTypes::VerifyHashingNotEnabled))
        }

        Ok(Self {
            header_size: DEFAULT_HEADER_SIZE,
            data_size: DATA_SIZES[DEFAULT_DATA_SIZE],
            v_hash: true,
        })
    }

    pub fn new_custom(header_size: u64, data_size: u8, v_hash: bool) -> Result<Self, SnowBinError> {
        let data_size = match data_size {
            8 => 8,
            16 => 16,
            32 => 32,
            64 => 64,
            /*128 => 128,*/
            _ => return Err(SnowBinError::new(SnowBinErrorTypes::DataSizeNotAllowed)),
        };

        if !VERIFY_HASHING && v_hash {
            return Err(SnowBinError::new(SnowBinErrorTypes::VerifyHashingNotEnabled));
        }

        Ok(Self {
            header_size,
            data_size,
            v_hash,
        })
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
        if !VERIFY_HASHING && info.v_hash {
            return Err(SnowBinError::new(SnowBinErrorTypes::VerifyHashingNotEnabled));
        }

        let mut file = match File::create(path) {
            Ok(file) => file,
            Err(_) => return Err(SnowBinError::new(SnowBinErrorTypes::CouldNotCreateOrOpenFile)),
        };

        SnowBinWriter::init_file(&mut file, info)?;

        Ok(Self {
            info: info.clone(),
            file,
            done: false
        })
    }

    fn init_file(file: &mut File, info: &SnowBinInfo) -> Result<(), SnowBinError> {
        file.seek(SeekFrom::Start(0)).map_err(|_| SnowBinError::new(SnowBinErrorTypes::IOWriteError))?;

        writer::write_header(file, "SNOW_BIN", 8)?;
        writer::write_u64(file, VERSION)?;
        writer::write_u64(file, info.header_size)?;
        writer::write_u8(file, info.data_size)?;
        writer::write_bool(file, info.v_hash)
    }

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
            if !VERIFY_HASHING && self.info.v_hash {
                return Err(SnowBinError::new(SnowBinErrorTypes::VerifyHashingNotEnabled));
            }

            // Write Data
            writer::write_header(&mut self.file, header, self.info.header_size)?;
            match self.info.data_size {
                8 => writer::write_u8(&mut self.file, data.len() as u8)?,
                16 => writer::write_u16(&mut self.file, data.len() as u16)?,
                32 => writer::write_u32(&mut self.file, data.len() as u32)?,
                64 => writer::write_u64(&mut self.file, data.len() as u64)?,
                /*128 => 128,*/
                _ => return Err(SnowBinError::new(SnowBinErrorTypes::DataSizeNotAllowed)),
            }
            writer::write_bytes(&mut self.file, data)?;

            // Verify Hash
            #[cfg(feature = "v_hash")]
            {
                if self.info.v_hash {
                    let hash = seahash::hash(data);
                    writer::write_u64(&mut self.file, hash)?;
                }
            }

            return Ok(()); //TODO
        }

        Err(SnowBinError::new(SnowBinErrorTypes::IOWriterClosed))
    }

    fn get_max_size(&self) -> Result<u64, SnowBinError> {
        Ok(match self.info.data_size {
            8 => u8::MAX as u64,
            16 => u16::MAX as u64,
            32 => u32::MAX as u64,
            64 => u64::MAX as u64,
            /*128 => 128,*/
            _ => return Err(SnowBinError::new(SnowBinErrorTypes::DataSizeNotAllowed)),
        })
    }

    pub fn close(&mut self) -> Result<(), SnowBinError> {
        use std::io::Write;

        if !self.done {
            writer::write_header(&mut self.file, "SNOW_END", self.info.header_size)?;
            self.file.flush().map_err(|_| SnowBinError::new(SnowBinErrorTypes::IOWriteError))?;

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
    data_start: u64,
}
// Read
// Read_Assert

pub fn verify_hashing_allowed() -> bool {
    VERIFY_HASHING
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
