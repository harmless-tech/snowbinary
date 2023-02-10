use std::{fs::File, io::Write};

use crate::error::{SnowBinError, SnowBinErrorTypes};

pub fn error(result: std::io::Result<()>) -> Result<(), SnowBinError> {
    match result {
        Ok(r) => Ok(r),
        Err(_) => Err(SnowBinError::new(SnowBinErrorTypes::IOWriteError)),
    }
}

pub fn write_header(file: &mut File, header: &str, header_len: u64) -> Result<(), SnowBinError> {
    if header.len() as u64 > header_len {
        return Err(SnowBinError::new(SnowBinErrorTypes::HeaderTooLong));
    }

    let null_buffer = vec![0_u8; (header_len - header.len() as u64) as usize];
    let buffer = &[header.as_bytes(), &null_buffer].concat();

    error(file.by_ref().write_all(buffer))
}

pub fn write_bytes(file: &mut File, data: &[u8]) -> Result<(), SnowBinError> {
    error(file.by_ref().write_all(data))
}

pub fn write_u8(file: &mut File, data: u8) -> Result<(), SnowBinError> {
    let buffer = &(data).to_be_bytes();
    error(file.by_ref().write_all(buffer))
}

pub fn write_u16(file: &mut File, data: u16) -> Result<(), SnowBinError> {
    let buffer = &(data).to_be_bytes();
    error(file.by_ref().write_all(buffer))
}

pub fn write_u32(file: &mut File, data: u32) -> Result<(), SnowBinError> {
    let buffer = &(data).to_be_bytes();
    error(file.by_ref().write_all(buffer))
}

pub fn write_u64(file: &mut File, data: u64) -> Result<(), SnowBinError> {
    let buffer = &(data).to_be_bytes();
    error(file.by_ref().write_all(buffer))
}

pub fn write_bool(file: &mut File, data: bool) -> Result<(), SnowBinError> {
    let buffer = &(data as u8).to_be_bytes();
    error(file.by_ref().write_all(buffer))
}
