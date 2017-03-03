use super::ops;
use super::super::source;
use super::super::types;

#[derive(Clone, Debug, PartialEq)]
pub struct Const(Vec<ops::Instruction>, source::SourceKey);

#[derive(Clone, Debug, PartialEq)]
pub struct Global {
    gtype: types::GlobalType,
    value: Const,
    source: source::SourceKey,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Func {
    ftype: ops::Var,
    locals: Vec<types::ValueType>,
    body: Vec<ops::Instruction>,
    source: source::SourceKey,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Table {
    ttype: types::TableType,
    source: source::SourceKey,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Memory {
    mtypes: types::MemoryType,
    source: source::SourceKey,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Segment<T> {
    index: ops::Var,
    offset: Const,
    source: source::SourceKey,
    init: T,
}

pub type TableSegment = Segment<Vec<ops::Var>>;
pub type MemorySegment = Segment<String>;

/*
* Globals & Functions *)

type 'data segment = 'data segment' Source.phrase
and 'data segment' =
{
  index : var;
  offset : const;
  init : 'data;
}

type table_segment = var list segment
type memory_segment = string segment
*/
