mod default_tests {
    use std::path::PathBuf;

    use crate::{SnowBinError, SnowBinInfo, SnowBinReader, SnowBinWriter};

    #[test]
    fn info_test() -> Result<(), SnowBinError> {
        SnowBinInfo::default();

        SnowBinInfo::new(8, 8)?;
        SnowBinInfo::new(8, 16)?;
        SnowBinInfo::new(8, 32)?;
        SnowBinInfo::new(8, 64)?;
        SnowBinInfo::new(34_785_382, 8)?;
        SnowBinInfo::new(7_543_454, 16)?;
        SnowBinInfo::new(7_843_463, 32)?;
        SnowBinInfo::new(45_646_234, 64)?;
        SnowBinInfo::new(u32::MAX, 8)?;
        SnowBinInfo::new(u32::MAX, 16)?;
        SnowBinInfo::new(u32::MAX, 32)?;
        SnowBinInfo::new(u32::MAX, 64)?;

        assert_eq!(
            SnowBinInfo::new(1, 8).unwrap_err(),
            SnowBinError::HeaderSizeTooSmall
        );
        assert_eq!(
            SnowBinInfo::new(1, 16).unwrap_err(),
            SnowBinError::HeaderSizeTooSmall
        );
        assert_eq!(
            SnowBinInfo::new(1, 32).unwrap_err(),
            SnowBinError::HeaderSizeTooSmall
        );
        assert_eq!(
            SnowBinInfo::new(1, 64).unwrap_err(),
            SnowBinError::HeaderSizeTooSmall
        );
        assert_eq!(
            SnowBinInfo::new(1, 1).unwrap_err(),
            SnowBinError::HeaderSizeTooSmall
        );
        assert_eq!(
            SnowBinInfo::new(8, 1).unwrap_err(),
            SnowBinError::DataSizeNotAllowed
        );
        assert_eq!(
            SnowBinInfo::new(8, u8::MAX).unwrap_err(),
            SnowBinError::DataSizeNotAllowed
        );

        Ok(())
    }

    #[test]
    fn h_test() -> Result<(), SnowBinError> {
        {
            let info = SnowBinInfo::new(8, 64)?;
            let mut writer = SnowBinWriter::new(info, PathBuf::from("./file.temp"))?;

            writer.write("TEST", b"This is a String!")?;
            writer.write("TEST321", b"This is a String!")?;
            writer.write("Header", b"This is for doc tests!")?;

            writer.close()?;
        }

        {
            let mut reader = SnowBinReader::new(PathBuf::from("./file.temp"))?;

            reader.read("TEST").unwrap();
            assert_eq!(
                reader.read("NULL_NO").unwrap_err(),
                SnowBinError::ReachedEOF
            );
        }

        Ok(())
    }
}
