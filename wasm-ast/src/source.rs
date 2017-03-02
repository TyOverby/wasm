pub enum FileLocation {
    SourceLocation {
        line: usize,
        column: usize,
    },
    BinaryLocation(usize),
}

pub struct Pos {
    pub file: String,
    pub location: FileLocation
}

pub struct Region {
    pub left: Pos,
    pub right: Pos,
}
