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
    pub name: String,
    pub kind: ExportKind,
    pub item: ops::Var,
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
    pub module_name: String,
    pub item_name: String,
    pub kind: ImportKind,
}

#[derive(Clone, PartialEq, Debug)]
pub struct Module {
    pub types: Vec<types::FuncType>,
    pub globals: Vec<misc::Global>,
    pub tables: Vec<misc::Table>,
    pub memories: Vec<misc::Memory>,
    pub funcs: Vec<misc::Func>,
    pub start: Option<ops::Var>,
    pub elems: Vec<misc::Segment<Vec<ops::Var>>>,
    pub data: Vec<misc::Segment<String>>,
    pub imports: Vec<Import>,
    pub exports: Vec<Export>,
}

impl Default for Module {
    fn default() -> Module {
        Module {
            types: vec![],
            globals: vec![],
            tables: vec![],
            memories: vec![],
            funcs: vec![],
            start: None,
            elems: vec![],
            data: vec![],
            imports: vec![],
            exports: vec![],
        }
    }
}
