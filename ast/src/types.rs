#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
pub enum ValueType {
    I32, I64, F32, F64
}

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
pub enum ElemType {
    AnyFuncType
}

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub struct StackType(Vec<ValueType>);

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub struct FuncType(StackType, StackType);

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
pub struct Limits<T> {
    min: T,
    max: T,
}

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
pub enum Mutability {
    Immutable, Mutable
}

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
pub struct TableType(Limits<i32>, ElemType);

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
pub struct MemoryType(Limits<i32>);

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
pub struct GlobalType(ValueType, Mutability);

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub enum ExternalType {
    ExternalFuncType(FuncType),
    ExternalTableType(TableType),
    ExternalMemoryType(MemoryType),
    ExternalGlobalType(GlobalType),
}

impl ValueType {
    pub fn size_of(self) -> u8 {
        match self {
            ValueType::I32 | ValueType::F32 => 4,
            ValueType::I64 | ValueType::F64 => 8,
        }
    }
}
