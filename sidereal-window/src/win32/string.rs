/// Convert to PCWSTR from string.
pub trait ToUTF16String {
    fn to_utf16(&self) -> Vec<u16>;
}

impl ToUTF16String for String {
    fn to_utf16(&self) -> Vec<u16> {
        self.encode_utf16()
            .chain([0u16])
            .collect::<Vec<u16>>()
    }
}
