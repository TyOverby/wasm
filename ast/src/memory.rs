pub type PageSize = i32;
pub type Address = i64;
pub type Offset = i32;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum MemSize {
    Mem8, Mem16, Mem32
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum Extension { SignExtend, ZeroExtend }

pub const PAGE_SIZE: usize = 0x10000usize;
