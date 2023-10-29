use strum::EnumIter;
use strum::EnumString;

#[derive(serde::Serialize, Clone, Copy, EnumString, EnumIter, strum::Display, Debug, PartialEq, serde::Deserialize)]
#[strum(serialize_all = "kebab_case")]
#[serde(rename_all = "kebab-case")]
#[repr(u64)]
pub enum KnownMagic {
    /// Prefixes every rain meta document "18377652714897045620"
    RainMetaDocumentV1 = 0xff0a89c674ee7874,

    /// Solidity ABI meta v2 "18439425400648969438"
    SolidityAbiV2 = 0xffe5ffb4a3ff2cde,

    /// Ops meta v1 "18439188432805991860"
    OpMetaV1 = 0xffe5282f43e495b4,

    /// InterpreterCaller meta v1 "18429323134567717275"
    InterpreterCallerMetaV1 = 0xffc21bbf86cc199b,

    /// Authroing meta v1 "18440520426328744501"
    AuthoringMetaV1 = 0xffe9e3a02ca8e235,

    /// Dotrain meta v1 "18436262373317404820"
    DotrainV1 = 0xffdac2f2f37be894,

    /// Rainlang meta v1 "18382595872068946087"
    RainlangV1 = 0xff1c198cec3b48a7,

    /// ExpressionDeployer deployed bytecode meta v1 "18436497220406627634"
    ExpressionDeployerV2BytecodeV1 = 0xffdb988a8cd04d32,
}

impl KnownMagic {
    pub fn to_prefix_bytes(&self) -> [u8; 8] {
        // Use big endian here as the magic numbers are for binary data prefixes.
        (*self as u64).to_be_bytes()
    }

    pub fn from_u64(value: u64) -> anyhow::Result<KnownMagic> {
        match value {
            v if v == KnownMagic::AuthoringMetaV1 as u64 => Ok(KnownMagic::AuthoringMetaV1),
            v if v == KnownMagic::DotrainV1 as u64 => Ok(KnownMagic::DotrainV1),
            v if v == KnownMagic::ExpressionDeployerV2BytecodeV1 as u64 => Ok(KnownMagic::ExpressionDeployerV2BytecodeV1),
            v if v == KnownMagic::InterpreterCallerMetaV1 as u64 => Ok(KnownMagic::InterpreterCallerMetaV1),
            v if v == KnownMagic::OpMetaV1 as u64 => Ok(KnownMagic::OpMetaV1),
            v if v == KnownMagic::RainMetaDocumentV1 as u64 => Ok(KnownMagic::RainMetaDocumentV1),
            v if v == KnownMagic::RainlangV1 as u64 => Ok(KnownMagic::RainlangV1),
            v if v == KnownMagic::SolidityAbiV2 as u64 => Ok(KnownMagic::SolidityAbiV2),
            _ => Err(anyhow::anyhow!("unknown magic number"))
        }
    }
}

impl Into<KnownMagic> for u64 {
    fn into(self) -> KnownMagic {
        match self {
            v if v == KnownMagic::AuthoringMetaV1 as u64 => KnownMagic::AuthoringMetaV1,
            v if v == KnownMagic::DotrainV1 as u64 => KnownMagic::DotrainV1,
            v if v == KnownMagic::ExpressionDeployerV2BytecodeV1 as u64 => KnownMagic::ExpressionDeployerV2BytecodeV1,
            v if v == KnownMagic::InterpreterCallerMetaV1 as u64 => KnownMagic::InterpreterCallerMetaV1,
            v if v == KnownMagic::OpMetaV1 as u64 => KnownMagic::OpMetaV1,
            v if v == KnownMagic::RainMetaDocumentV1 as u64 => KnownMagic::RainMetaDocumentV1,
            v if v == KnownMagic::RainlangV1 as u64 => KnownMagic::RainlangV1,
            v if v == KnownMagic::SolidityAbiV2 as u64 => KnownMagic::SolidityAbiV2,
            _ => panic!("unknown magic number")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::KnownMagic;

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
}
