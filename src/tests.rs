//TODO Writing and Reading tests.

mod default_tests {
    use crate::{SnowBinError, SnowBinInfo};

    #[test]
    fn info_test() -> Result<(), SnowBinError> {
        SnowBinInfo::default();

        SnowBinInfo::new(8, 8)?;
        SnowBinInfo::new(8, 16)?;
        SnowBinInfo::new(8, 32)?;
        SnowBinInfo::new(8, 64)?;
        SnowBinInfo::new(34785382, 8)?;
        SnowBinInfo::new(755463454, 16)?;
        SnowBinInfo::new(7864263463, 32)?;
        SnowBinInfo::new(45662346234, 64)?;
        SnowBinInfo::new(u64::MAX, 8)?;
        SnowBinInfo::new(u64::MAX, 16)?;
        SnowBinInfo::new(u64::MAX, 32)?;
        SnowBinInfo::new(u64::MAX, 64)?;

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
}

#[cfg(feature = "v_hash")]
mod v_hash_tests {
    use crate::{SnowBinError, SnowBinInfo};

    #[test]
    fn info_test() -> Result<(), SnowBinError> {
        SnowBinInfo::default_with_v_hash();

        SnowBinInfo::new_with_v_hash(8, 8)?;
        SnowBinInfo::new_with_v_hash(8, 16)?;
        SnowBinInfo::new_with_v_hash(8, 32)?;
        SnowBinInfo::new_with_v_hash(8, 64)?;
        SnowBinInfo::new_with_v_hash(34785382, 8)?;
        SnowBinInfo::new_with_v_hash(755463454, 16)?;
        SnowBinInfo::new_with_v_hash(7864263463, 32)?;
        SnowBinInfo::new_with_v_hash(45662346234, 64)?;
        SnowBinInfo::new_with_v_hash(u64::MAX, 8)?;
        SnowBinInfo::new_with_v_hash(u64::MAX, 16)?;
        SnowBinInfo::new_with_v_hash(u64::MAX, 32)?;
        SnowBinInfo::new_with_v_hash(u64::MAX, 64)?;

        assert_eq!(
            SnowBinInfo::new_with_v_hash(1, 8).unwrap_err(),
            SnowBinError::HeaderSizeTooSmall
        );
        assert_eq!(
            SnowBinInfo::new_with_v_hash(1, 16).unwrap_err(),
            SnowBinError::HeaderSizeTooSmall
        );
        assert_eq!(
            SnowBinInfo::new_with_v_hash(1, 32).unwrap_err(),
            SnowBinError::HeaderSizeTooSmall
        );
        assert_eq!(
            SnowBinInfo::new_with_v_hash(1, 64).unwrap_err(),
            SnowBinError::HeaderSizeTooSmall
        );
        assert_eq!(
            SnowBinInfo::new_with_v_hash(1, 1).unwrap_err(),
            SnowBinError::HeaderSizeTooSmall
        );
        assert_eq!(
            SnowBinInfo::new_with_v_hash(8, 1).unwrap_err(),
            SnowBinError::DataSizeNotAllowed
        );
        assert_eq!(
            SnowBinInfo::new_with_v_hash(8, u8::MAX).unwrap_err(),
            SnowBinError::DataSizeNotAllowed
        );

        Ok(())
    }
}
