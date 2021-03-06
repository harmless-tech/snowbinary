use crate::error::{SnowBinError, SnowBinErrorTypes};
use std::{convert::TryInto, fs::File, io::Read, mem};

pub fn error(result: std::io::Result<usize>) -> Result<usize, SnowBinError> {
    match result {
        Ok(r) => Ok(r),
        Err(_) => Err(SnowBinError::new(SnowBinErrorTypes::IOReadError)),
    }
}

pub fn read_header(file: &mut File, header_len: u64) -> Result<String, SnowBinError> {
    let mut buffer = vec![32_u8; header_len as usize];
    error(file.by_ref().take(header_len).read(&mut buffer))?;

    Ok(String::from_utf8(buffer)
        .map_err(|_| SnowBinError::new(SnowBinErrorTypes::MalformedHeader))?)
}

pub fn read_bytes(file: &mut File, length: u64) -> Result<Vec<u8>, SnowBinError> {
    let mut buffer = vec![0_u8; length as usize];
    error(file.by_ref().take(length).read(&mut buffer))?;

    Ok(buffer)
}

pub fn read_u8(file: &mut File) -> Result<u8, SnowBinError> {
    let mut buffer = [0_u8; 1];
    error(file.by_ref().take(1).read(&mut buffer))?;

    let (uint_bytes, _) = buffer.split_at(mem::size_of::<u8>());
    let uint = u8::from_be_bytes(
        uint_bytes
            .try_into()
            .map_err(|_| SnowBinError::new(SnowBinErrorTypes::MalformedUInt))?,
    );

    Ok(uint)
}

pub fn read_u16(file: &mut File) -> Result<u16, SnowBinError> {
    let mut buffer = [0_u8; 2];
    error(file.by_ref().take(2).read(&mut buffer))?;

    let (uint_bytes, _) = buffer.split_at(mem::size_of::<u16>());
    let uint = u16::from_be_bytes(
        uint_bytes
            .try_into()
            .map_err(|_| SnowBinError::new(SnowBinErrorTypes::MalformedUInt))?,
    );

    Ok(uint)
}

pub fn read_u32(file: &mut File) -> Result<u32, SnowBinError> {
    let mut buffer = [0_u8; 4];
    error(file.by_ref().take(4).read(&mut buffer))?;

    let (uint_bytes, _) = buffer.split_at(mem::size_of::<u32>());
    let uint = u32::from_be_bytes(
        uint_bytes
            .try_into()
            .map_err(|_| SnowBinError::new(SnowBinErrorTypes::MalformedUInt))?,
    );

    Ok(uint)
}

pub fn read_u64(file: &mut File) -> Result<u64, SnowBinError> {
    let mut buffer = [0_u8; 8];
    error(file.by_ref().take(8).read(&mut buffer))?;

    let (uint_bytes, _) = buffer.split_at(mem::size_of::<u64>());
    let uint = u64::from_be_bytes(
        uint_bytes
            .try_into()
            .map_err(|_| SnowBinError::new(SnowBinErrorTypes::MalformedUInt))?,
    );

    Ok(uint)
}

pub fn read_bool(file: &mut File) -> Result<bool, SnowBinError> {
    let byte = read_u8(file)?;
    Ok(match byte {
        0 => false,
        _ => true,
    })
}
