use super::super::types;
use super::super::memory;
use super::super::values;
use super::super::source;

pub mod intop {
    #[derive(Copy, Clone, Debug, Eq, PartialEq)]
    pub enum UnOp {
        Clz, Ctz, Popcnt,
    }

    #[derive(Copy, Clone, Debug, Eq, PartialEq)]
    pub enum BinOp {
        Add, Sub, Mul, DivS, DivU, RemS,
        RemU, And, Or, Xor, Shl,
        ShrS, ShrU, Rotl, Rotr,
    }

    #[derive(Copy, Clone, Debug, Eq, PartialEq)]
    pub enum TestOp {
        Eqz,
    }

    #[derive(Copy, Clone, Debug, Eq, PartialEq)]
    pub enum RelOp {
        Eq, Ne, LtS, LtU, GtS,
        GtU, LeS, LeU, GeS, GeU,
    }

    #[derive(Copy, Clone, Debug, Eq, PartialEq)]
    pub enum CvtOp {
        ExtendSI32, ExtendUI32, WrapI64,
        TruncSF32, TruncUF32, TruncSF64,
        TruncUF64, ReinterpretFloat,
    }
}

pub mod floatop {
    #[derive(Copy, Clone, Debug, Eq, PartialEq)]
    pub enum UnOp {
        Neg, Abs, Ceil, Floor, Trunc, Nearest, Sqrt,
    }

    #[derive(Copy, Clone, Debug, Eq, PartialEq)]
    pub enum BinOp {
        Add, Sub, Mul, Div, Min, Max, CopySign,
    }

    #[derive(Copy, Clone, Debug, Eq, PartialEq)]
    pub enum TestOp {}

    #[derive(Copy, Clone, Debug, Eq, PartialEq)]
    pub enum RelOp {
        Eq, Ne, Lt, Gt, Le, Ge,
    }

    #[derive(Copy, Clone, Debug, Eq, PartialEq)]
    pub enum CvtOp {
        ConvertSI32, ConvertUI32, ConvertSI64, ConvertUI64,
        PromoteF32, DemoteF64, ReinterpretInt,
    }
}

pub type UnOp = values::Values<intop::UnOp, intop::UnOp, floatop::UnOp, floatop::UnOp>;
pub type BinOp = values::Values<intop::BinOp, intop::BinOp, floatop::BinOp, floatop::BinOp>;
pub type TestOp = values::Values<intop::TestOp, intop::TestOp, floatop::TestOp, floatop::TestOp>;
pub type RelOp = values::Values<intop::RelOp, intop::RelOp, floatop::RelOp, floatop::RelOp>;
pub type CvtOp = values::Values<intop::CvtOp, intop::CvtOp, floatop::CvtOp, floatop::CvtOp>;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct MemOp<T> {
    pub typ: types::ValueType,
    pub align: u32,
    pub offset: memory::Offset,
    pub size: Option<T>
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct LoadOp(pub MemOp<(memory::PageSize, memory::Extension)>);

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct StoreOp(pub MemOp<memory::PageSize>);

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Var(pub i32, pub source::SourceKey);

#[derive(Clone, Debug, PartialEq)]
pub struct Literal(pub values::Value, pub source::Pos);

#[derive(Clone, Debug, PartialEq)]
pub struct Instruction(pub Instr, pub source::SourceKey);

#[derive(Clone, Debug, PartialEq)]
pub enum Instr {
    Unreachable,
    Nop,
    Block(types::StackType, Vec<Instr>),
    Loop(types::StackType, Vec<Instr>),
    If(types::StackType, Vec<Instr>, Vec<Instr>),
    Br(Var),
    BrIf(Var),
    BrTable(Vec<Var>),
    Return,
    Call(Var),
    CallIndirect(Var),
    Drop,
    Select,
    GetLocal(Var),
    SetLocal(Var),
    TeeLocal(Var),
    GetGlobal(Var),
    SetGlobal(Var),
    Load(LoadOp),
    Store(StoreOp),
    CurrentMemory,
    GrowMemory,
    Const(Literal),
    Test(TestOp),
    Compare(RelOp),
    Unary(UnOp),
    Binary(BinOp),
    Convert(CvtOp),
}
