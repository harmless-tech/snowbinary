# SnowBinary
[![Rust Checks](https://github.com/harmless-tech/snowbinary/actions/workflows/checks.yml/badge.svg)](https://github.com/harmless-tech/snowbinary/actions/workflows/checks.yml)
[![Rust Build and Test](https://github.com/harmless-tech/snowbinary/actions/workflows/build.yml/badge.svg)](https://github.com/harmless-tech/snowbinary/actions/workflows/build.yml)
[![Rust Build, Test, and Release](https://github.com/harmless-tech/snowbinary/actions/workflows/release.yml/badge.svg)](https://github.com/harmless-tech/snowbinary/actions/workflows/release.yml)
[![Crates.io](https://img.shields.io/crates/v/snowbinary)](https://crates.io/crates/snowbinary)
[![docs.rs](https://img.shields.io/docsrs/snowbinary/latest)](https://docs.rs/snowbinary/latest/snowbinary)
[![rustc-msrv](https://img.shields.io/badge/rustc-1.66%2B-blue?logo=rust)](https://www.rust-lang.org/tools/install)

A basic header based binary file format, writer and reader.

## Basic Example

```rust
{
    let info = SnowBinInfo::default();    

    let writer = SnowBinWriter::new(info, PATH as PathBuf)?;
    writer.write("Header", DATA as &[u8])?;
} // File is flushed when writer.close() is called or when writer is dropped.

{
    let reader = SnowBinReader::new()?;
    reader.read("Header")?; // Returns data has Vec<u8>
}
```

## Binary Format (Supported by this version)

#### Spec 2

- Default Max Header Size: 8 bytes.
- Default Max Data Size: u64.


- Start with an 8 byte header of "SNOW_BIN".
- Then 8 bytes showing the snow binary version. (Spec Version) (u64)
- Then 4 bytes showing the max header size in bytes. (At least 8 bytes, max of u32::MAX bytes) (u32)
- Then 1 byte showing the max data size.  (u8::MAX, u16::MAX, u32::MAX, u64::MAX) (u8)
- Then write data:
  - Header of MAX_HEADER_SIZE. (No conflicting header names. This is not checked by the writer or reader.)
  - Data size of MAX_DATA_SIZE.
  - Data.
  - Repeat until \\/.
- End with a MAX_HEADER_SIZE header of "SNOW_END".
- 32 byte verification hash. (Using blake3)
