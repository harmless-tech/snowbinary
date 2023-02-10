use std::{fs::File, io::Write};

use crate::error::SnowBinError;

#[inline]
pub fn error(result: std::io::Result<()>) -> Result<(), SnowBinError> {
    match result {
        Ok(r) => Ok(r),
        Err(_) => Err(SnowBinError::IOWriteError),
    }
}

pub fn write_header(
    file: &mut File,
    header: &str,
    header_len: u32,
) -> Result<Vec<u8>, SnowBinError> {
    if header.len() as u32 > header_len {
        return Err(SnowBinError::HeaderTooLong);
    }

    let null_buffer = vec![32_u8; (header_len - header.len() as u32) as usize];
    let buffer = [header.as_bytes(), &null_buffer].concat();

    error(file.by_ref().write_all(&buffer))?;

    Ok(buffer)
}

pub fn write_bytes(file: &mut File, data: &[u8]) -> Result<(), SnowBinError> {
    error(file.by_ref().write_all(data))
}

pub fn write_u8(file: &mut File, data: u8) -> Result<(), SnowBinError> {
    let buffer = &(data).to_le_bytes();
    error(file.by_ref().write_all(buffer))
}

pub fn write_u16(file: &mut File, data: u16) -> Result<(), SnowBinError> {
    let buffer = &(data).to_le_bytes();
    error(file.by_ref().write_all(buffer))
}

pub fn write_u32(file: &mut File, data: u32) -> Result<(), SnowBinError> {
    let buffer = &(data).to_le_bytes();
    error(file.by_ref().write_all(buffer))
}

pub fn write_u64(file: &mut File, data: u64) -> Result<(), SnowBinError> {
    let buffer = &(data).to_le_bytes();
    error(file.by_ref().write_all(buffer))
}
