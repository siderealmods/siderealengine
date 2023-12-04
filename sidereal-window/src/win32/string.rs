use windows::core::PCWSTR;

/// Convert to PCWSTR from string.
pub trait IntoPCWSTR {
    fn into_pcwstr(&self) -> PCWSTR;
}

impl IntoPCWSTR for String {
    fn into_pcwstr(&self) -> PCWSTR {
        let mut encoded = self
            .encode_utf16()
            .chain([0u16])
            .collect::<Vec<u16>>();

        PCWSTR(encoded.as_mut_ptr())
    }
}
