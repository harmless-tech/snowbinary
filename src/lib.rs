use crate::error::{SnowBinError, SnowBinErrorTypes};
use std::{fs::File, io::Error, path::PathBuf};

pub mod error;

const VERSION: u32 = 0; // Snow Binary File Format

const MIN_VERSION: u32 = 0;
const DEFAULT_HEADER_SIZE: u32 = 8;
const DATA_SIZES: [u8; 5] = [8, 16, 32, 64, 128];
const DEFAULT_DATA_SIZE: usize = 3;

#[cfg(feature = "v_hash")]
const VERIFY_HASHING: bool = true;
#[cfg(not(feature = "v_hash"))]
const VERIFY_HASHING: bool = false;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct SnowBinInfo {
    header_size: u32,
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
            return Err(SnowBinError::new(SnowBinErrorTypes::VerifyHashingNotEnabled, ))
        }

        Ok(Self {
            header_size: DEFAULT_HEADER_SIZE,
            data_size: DATA_SIZES[DEFAULT_DATA_SIZE],
            v_hash: true,
        })
    }

    pub fn new_custom(header_size: u32, data_size: u8, v_hash: bool) -> Result<Self, SnowBinError> {
        let data_size = match data_size {
            8 => 8,
            16 => 16,
            32 => 32,
            64 => 64,
            128 => 128,
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
}
impl SnowBinWriter {
    pub fn new(info: &SnowBinInfo, path: PathBuf) -> Result<Self, SnowBinError> {
        let file = match File::create(path) {
            Ok(file) => file,
            Err(_) => return Err(SnowBinError::new(SnowBinErrorTypes::CouldNotCreateOrOpenFile)),
        };

        SnowBinWriter::init_file(&file); //TODO Errors???

        Ok(Self {
            info: info.clone(),
            file,
        })
    }

    fn init_file(file: &File) {}
}

#[derive(Debug)]
pub struct SnowBinReader {
    info: SnowBinInfo,
}

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
