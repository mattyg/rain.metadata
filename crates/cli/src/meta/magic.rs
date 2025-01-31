/// All known Rain magic numbers
#[derive(
    serde::Serialize,
    Clone,
    Copy,
    strum::EnumIter,
    strum::EnumString,
    strum::Display,
    Debug,
    PartialEq,
    serde::Deserialize,
)]
#[strum(serialize_all = "kebab_case")]
#[serde(rename_all = "kebab-case")]
#[repr(u64)]
pub enum KnownMagic {
    /// Prefixes every rain meta document
    RainMetaDocumentV1 = 0xff0a89c674ee7874,

    /// Ops meta v1
    OpMetaV1 = 0xffe5282f43e495b4,
    /// Dotrain meta v1
    DotrainV1 = 0xffdac2f2f37be894,
    /// Rainlang meta v1
    RainlangV1 = 0xff1c198cec3b48a7,
    /// Solidity ABI meta v2
    SolidityAbiV2 = 0xffe5ffb4a3ff2cde,
    /// Authroing meta v1
    AuthoringMetaV1 = 0xffe9e3a02ca8e235,
    /// InterpreterCaller meta v1
    InterpreterCallerMetaV1 = 0xffc21bbf86cc199b,
    /// ExpressionDeployer deployed bytecode meta v1
    ExpressionDeployerV2BytecodeV1 = 0xffdb988a8cd04d32,
}

impl KnownMagic {
    pub fn to_prefix_bytes(&self) -> [u8; 8] {
        // Use big endian here as the magic numbers are for binary data prefixes.
        (*self as u64).to_be_bytes()
    }
}

impl TryFrom<u64> for KnownMagic {
    type Error = crate::error::Error;
    fn try_from(value: u64) -> Result<Self, Self::Error> {
        match value {
            v if v == KnownMagic::OpMetaV1 as u64 => Ok(KnownMagic::OpMetaV1),
            v if v == KnownMagic::DotrainV1 as u64 => Ok(KnownMagic::DotrainV1),
            v if v == KnownMagic::RainlangV1 as u64 => Ok(KnownMagic::RainlangV1),
            v if v == KnownMagic::SolidityAbiV2 as u64 => Ok(KnownMagic::SolidityAbiV2),
            v if v == KnownMagic::AuthoringMetaV1 as u64 => Ok(KnownMagic::AuthoringMetaV1),
            v if v == KnownMagic::RainMetaDocumentV1 as u64 => Ok(KnownMagic::RainMetaDocumentV1),
            v if v == KnownMagic::InterpreterCallerMetaV1 as u64 => {
                Ok(KnownMagic::InterpreterCallerMetaV1)
            }
            v if v == KnownMagic::ExpressionDeployerV2BytecodeV1 as u64 => {
                Ok(KnownMagic::ExpressionDeployerV2BytecodeV1)
            }
            _ => Err(crate::error::Error::UnknownMagic),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::KnownMagic;
    use alloy_primitives::hex;

    #[test]
    fn test_rain_meta_document_v1() {
        let magic_number = KnownMagic::RainMetaDocumentV1;
        let magic_number_after_prefix = magic_number.to_prefix_bytes();

        assert_eq!(hex::encode(magic_number_after_prefix), "ff0a89c674ee7874");
    }

    #[test]
    fn test_solidity_abi_v2() {
        let magic_number = KnownMagic::SolidityAbiV2;
        let magic_number_after_prefix = magic_number.to_prefix_bytes();

        assert_eq!(hex::encode(magic_number_after_prefix), "ffe5ffb4a3ff2cde");
    }

    #[test]
    fn test_op_meta_v1() {
        let magic_number = KnownMagic::OpMetaV1;
        let magic_number_after_prefix = magic_number.to_prefix_bytes();

        assert_eq!(hex::encode(magic_number_after_prefix), "ffe5282f43e495b4");
    }

    #[test]
    fn test_interpreter_caller_meta_v1() {
        let magic_number = KnownMagic::InterpreterCallerMetaV1;
        let magic_number_after_prefix = magic_number.to_prefix_bytes();

        assert_eq!(hex::encode(magic_number_after_prefix), "ffc21bbf86cc199b");
    }

    #[test]
    fn test_authoring_meta_v1() {
        let magic_number = KnownMagic::AuthoringMetaV1;
        let magic_number_after_prefix = magic_number.to_prefix_bytes();

        assert_eq!(hex::encode(magic_number_after_prefix), "ffe9e3a02ca8e235");
    }

    #[test]
    fn test_dotrain_meta_v1() {
        let magic_number = KnownMagic::DotrainV1;
        let magic_number_after_prefix = magic_number.to_prefix_bytes();

        assert_eq!(hex::encode(magic_number_after_prefix), "ffdac2f2f37be894");
    }

    #[test]
    fn test_rainlang_meta_v1() {
        let magic_number = KnownMagic::RainlangV1;
        let magic_number_after_prefix = magic_number.to_prefix_bytes();

        assert_eq!(hex::encode(magic_number_after_prefix), "ff1c198cec3b48a7");
    }

    #[test]
    fn test_expression_deployer_v2_bytecode_meta_v1() {
        let magic_number = KnownMagic::ExpressionDeployerV2BytecodeV1;
        let magic_number_after_prefix = magic_number.to_prefix_bytes();

        assert_eq!(hex::encode(magic_number_after_prefix), "ffdb988a8cd04d32");
    }
}
