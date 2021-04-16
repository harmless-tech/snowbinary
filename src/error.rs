use std::{error::Error, fmt};

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum SnowBinErrorTypes {
    DataSizeNotAllowed,
    VerifyHashingNotEnabled,
    CouldNotCreateOrOpenFile,
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
                String::from("Data Size not 8, 16, 32, 64, or 128.")
            }
            SnowBinErrorTypes::VerifyHashingNotEnabled => {
                String::from("Verify hashing not enabled, please enable the feature.")
            }
            SnowBinErrorTypes::CouldNotCreateOrOpenFile => {
                String::from("Could not create or open the file.")
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
