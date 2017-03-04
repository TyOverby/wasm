use super::ops;
use super::super::source;
use super::super::types;

#[derive(Clone, Debug, PartialEq)]
pub struct Const(Vec<ops::Instruction>, source::SourceKey);

#[derive(Clone, Debug, PartialEq)]
pub struct Global {
    pub gtype: types::GlobalType,
    pub value: Const,
    pub source: source::SourceKey,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Func {
    pub ftype: ops::Var,
    pub locals: Vec<types::ValueType>,
    pub body: Vec<ops::Instruction>,
    pub source: source::SourceKey,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Table {
    pub ttype: types::TableType,
    pub source: source::SourceKey,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Memory {
    pub mtypes: types::MemoryType,
    pub source: source::SourceKey,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Segment<T> {
    pub index: ops::Var,
    pub offset: Const,
    pub source: source::SourceKey,
    pub init: T,
}

pub type TableSegment = Segment<Vec<ops::Var>>;
pub type MemorySegment = Segment<String>;
