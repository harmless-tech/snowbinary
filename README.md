# SnowBinary

A binary file format, writer, and reader.

## Binary Format

#### Spec 0 (Not Final)

- Default Max Header Size: 8 bytes.
- Default Max Data Size: u64.

<br>

- Start with an 8 byte header of "SNOW_BIN".
- Then 8 bytes showing the snow binary version. (Spec Version)
- Then 16 bytes showing the max header size in bytes. (At least 8 bytes, max of u32::MAX)
- Then 1 byte showing the max data size.  (u8::MAX, u16::MAX, u32::MAX, u64::MAX, u128::MAX)
- 1 byte showing if the data should have a verification hash.
- Then write data:
  - Header of MAX_HEADER_SIZE. (No conflicting header names)
  - Data size of MAX_DATA_SIZE.
  - Data.
  - An 8 byte verification hash, if enabled. (Only allowed if feature v_hash is enabled.)
  - Repeat until \/.
- End with a MAX_HEADER_SIZE byte header of "SNOW_END".

## Example

```rust

```
