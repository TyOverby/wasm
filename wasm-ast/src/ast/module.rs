use super::ops;
use super::super::types;
use super::misc;

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub enum ExportKind {
    FuncExport,
    TableExport,
    MemoryExport,
    GlobalExport,
}

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct Export {
    name: String,
    kind: ExportKind,
    item: ops::Var,
}

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub enum ImportKind {
    FuncImport,
    TableImport,
    MemoryImport,
    GLobalImport,
}

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct Import {
    module_name: String,
    item_name: String,
    kind: ImportKind,
}

#[derive(Clone, PartialEq, Debug)]
pub struct Module {
    types: Vec<types::FuncType>,
    globals: Vec<misc::Global>,
    tables: Vec<misc::Table>,
    memories: Vec<misc::Memory>,
    funcs: Vec<misc::Func>,
    start: Option<ops::Var>,
    elems: Vec<misc::Segment<Vec<ops::Var>>>,
    data: Vec<misc::Segment<String>>,
    imports: Vec<Import>,
    exports: Vec<Export>,
}
