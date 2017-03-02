#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
enum ValueType {
    I32, I64, F32, F64
}

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
enum ElemType {
    AnyFuncType
}

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
struct StackType(Vec<ValueType>);

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
struct FuncType(StackType, StackType);

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
struct Limits<T> {
    min: T,
    max: T,
}

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
enum Mutability {
    Immutable, Mutable
}

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
struct TableType(Limits<i32>, ElemType);

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
struct MemoryType(Limits<i32>);

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
struct GlobalType(ValueType, Mutability);

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
enum ExternalType {
    ExternalFuncType(FuncType),
    ExternalTableType(TableType),
    ExternalMemoryType(MemoryType),
    ExternalGlobalType(GlobalType),
}

impl ValueType {
    fn size_of(self) -> u8 {
        match self {
            ValueType::I32 | ValueType::F32 => 4,
            ValueType::I64 | ValueType::F64 => 8,
        }
    }
}
