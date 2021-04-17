use std::{
    fs::File,
    io::{Error, ErrorKind, Write},
};

pub fn write_header(file: &mut File, header: &str, header_len: u64) -> Result<(), Error> {
    if header.len() as u64 > header_len {
        return Err(Error::new(ErrorKind::Other, ""));
    }

    let null_buffer = vec![0_u8; (header_len - header.len() as u64) as usize];
    let buffer = &[header.as_bytes(), &null_buffer].concat();

    file.by_ref().write_all(buffer)
}

pub fn write_bytes(file: &mut File, data: &[u8]) -> Result<(), Error> {
    file.by_ref().write_all(data)
}

pub fn write_u8(file: &mut File, data: u8) -> Result<(), Error> {
    let buffer = &(data).to_be_bytes();
    file.by_ref().write_all(buffer)
}

pub fn write_u16(file: &mut File, data: u16) -> Result<(), Error> {
    let buffer = &(data).to_be_bytes();
    file.by_ref().write_all(buffer)
}

pub fn write_u32(file: &mut File, data: u32) -> Result<(), Error> {
    let buffer = &(data).to_be_bytes();
    file.by_ref().write_all(buffer)
}

pub fn write_u64(file: &mut File, data: u64) -> Result<(), Error> {
    let buffer = &(data).to_be_bytes();
    file.by_ref().write_all(buffer)
}

pub fn write_bool(file: &mut File, data: bool) -> Result<(), Error> {
    let buffer = &(data as u8).to_be_bytes();
    file.by_ref().write_all(buffer)
}
