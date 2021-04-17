mod writer;

extern crate libc;

use std::{
    ffi::CString,
    fs::File,
    io,
    io::{Seek, SeekFrom},
    os::raw::c_char,
};

pub const VERSION_SPEC: u64 = 0; // Snow Binary File Format

const DEFAULT_HEADER_SIZE: u64 = 8;
const DATA_SIZES: [u8; 4] = [8, 16, 32, 64];
const DEFAULT_DATA_SIZE: usize = 3;

const DATA_START: u64 = 26;

#[repr(C)]
pub struct SnowBinInfo {
    header_size: u64,
    data_size: u8,
    v_hash: bool,
}

#[repr(C)]
pub struct SnowBinWriter {
    info: SnowBinInfo,
    file: File,
    done: bool,
}

// SnowBinInfo
#[no_mangle]
pub unsafe extern "C" fn snow_bin_info_new() -> *mut SnowBinInfo {
    let b_info = Box::new(SnowBinInfo {
        header_size: DEFAULT_HEADER_SIZE,
        data_size: DATA_SIZES[DEFAULT_DATA_SIZE],
        v_hash: false,
    });
    Box::into_raw(b_info)
}

#[no_mangle]
pub unsafe extern "C" fn snow_bin_info_set_header_size(info: *mut SnowBinInfo, size: u64) -> bool {
    if info.is_null() {
        return false;
    }

    (*info).header_size = size;

    true
}

#[no_mangle]
pub unsafe extern "C" fn snow_bin_info_set_data_size(info: *mut SnowBinInfo, size: u8) -> bool {
    if info.is_null() {
        return false;
    }

    let data_size = match size {
        8 => 8,
        16 => 16,
        32 => 32,
        64 => 64,
        _ => return false,
    };

    (*info).data_size = data_size;

    return true;
}

#[no_mangle]
pub unsafe extern "C" fn snow_bin_info_set_v_hash(info: *mut SnowBinInfo, hash: bool) -> bool {
    if info.is_null() {
        return false;
    }

    (*info).v_hash = hash;

    return true;
}

#[no_mangle]
pub unsafe extern "C" fn snow_bin_info_del(info: *mut SnowBinInfo) {
    if info.is_null() {
        return;
    }

    libc::free(info as *mut libc::c_void);
}

// SnowBinWriter
#[no_mangle]
pub unsafe extern "C" fn snow_bin_writer_new(
    info: *const SnowBinInfo,
    path: *mut c_char,
) -> *mut SnowBinWriter {
    let path = match CString::from_raw(path).into_string() {
        Ok(s) => s,
        Err(_) => return std::ptr::null_mut(),
    };

    let mut file = match File::create(path) {
        Ok(file) => file,
        Err(_) => return std::ptr::null_mut(),
    };

    match init_file(&mut file, &info.read()) {
        Ok(_) => {}
        Err(_) => return std::ptr::null_mut(),
    }

    let b_writer = Box::new(SnowBinWriter {
        info: info.read(),
        file,
        done: false,
    });
    Box::into_raw(b_writer)
}

fn init_file(file: &mut File, info: &SnowBinInfo) -> Result<(), io::Error> {
    file.seek(SeekFrom::Start(0))?;

    writer::write_header(file, "SNOW_BIN", 8)?;
    writer::write_u64(file, VERSION_SPEC)?;
    writer::write_u64(file, info.header_size)?;
    writer::write_u8(file, info.data_size)?;
    writer::write_bool(file, info.v_hash)
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
