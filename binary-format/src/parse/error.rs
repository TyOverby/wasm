pub enum Error {
    InvalidValueType(u64),
    InvalidExternalKind(u8),
    Other(u32),
}
