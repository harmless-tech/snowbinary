# SnowBinary
[![Rust Build and Test](https://github.com/harmless-tech/snowbinary/actions/workflows/rust.yml/badge.svg)](https://github.com/harmless-tech/snowbinary/actions/workflows/rust.yml)
[![Rust Build, Test, and Create Release](https://github.com/harmless-tech/snowbinary/actions/workflows/release.yml/badge.svg)](https://github.com/harmless-tech/snowbinary/actions/workflows/release.yml)
![Crates.io](https://img.shields.io/crates/v/snowbinary)
![docs.rs](https://img.shields.io/docsrs/snowbinary/latest)

A binary file format, writer, and reader.

## Basic Example

```rust
{
    let info = SnowBinInfo::default();    

    let writer = SnowBinWriter::new(&info, PATH as PathBuf)?;
    writer.write("Header", DATA as &[u8])?;
} // File is flushed when writer.close() is called or when writer is dropped.

{
    let reader = SnowBinReader::new()?;
    reader.read("Header")?; // Returns data has Vec<u8>
}
```

## Future Improvements

- Better testing.
- More examples.
- Docs.

## Binary Format

#### Spec 1

- Default Max Header Size: 8 bytes.
- Default Max Data Size: u64.


- Start with an 8 byte header of "SNOW_BIN".
- Then 8 bytes showing the snow binary version. (Spec Version) (u64)
- Then 8 bytes showing the max header size in bytes. (At least 8 bytes, max of u64::MAX bytes) (u64)
- Then 1 byte showing the max data size.  (u8::MAX, u16::MAX, u32::MAX, u64::MAX) (u8)
- 1 byte showing if the data should have a verification hash. (u8)
- Then write data:
  - Header of MAX_HEADER_SIZE. (No conflicting header names. (This is not checked by the writer or reader.))
  - Data size of MAX_DATA_SIZE.
  - Data.
  - An 8 byte verification hash, if enabled. (Only allowed if feature v_hash is enabled.)
  - Repeat until \/.
- End with a MAX_HEADER_SIZE header of "SNOW_END".
