#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
pub enum FileLocation {
    SourceLocation {
        line: usize,
        column: usize,
    },
    BinaryLocation(usize),
}

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
pub struct Pos {
    pub file: String,
    pub location: FileLocation
}

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
pub struct Region {
    pub left: Pos,
    pub right: Pos,
}

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
pub struct SourceKey(u32);
