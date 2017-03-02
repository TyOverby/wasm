use super::types;
use super::memory;
use super::values;
use super::source;

pub mod intop {
    pub enum UnOp {
        Clz, Ctz, Popcnt,
    }

    pub enum BinOp {
        Add, Sub, Mul, DivS, DivU, RemS,
        RemU, And, Or, Xor, Shl,
        ShrS, ShrU, Rotl, Rotr,
    }

    pub enum TestOp {
        Eqz,
    }

    pub enum RelOp {
        Eq, Ne, LtS, LtU, GtS,
        GtU, LeS, LeU, GeS, GeU,
    }

    pub enum CvtOp {
        ExtendSI32, ExtendUI32, WrapI64,
        TruncSF32, TruncUF32, TruncSF64,
        TruncUF64, ReinterpretFloat,
    }
}

pub mod floatop {
    pub enum UnOp {
        Neg, Abs, Ceil, Floor, Trunc, Nearest, Sqrt,
    }
    pub enum BinOp {
        Add, Sub, Mul, Div, Min, Max, CopySign,
    }
    pub enum TestOp {}
    pub enum RelOp {
        Eq, Ne, Lt, Gt, Le, Ge,
    }
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

pub struct MemOp<T> {
    pub typ: types::ValueType,
    pub align: u32,
    pub offset: memory::Offset,
    pub size: Option<T>
}

pub struct LoadOp(MemOp<(memory::PageSize, memory::Extension)>);

pub struct StoreOp(MemOp<memory::PageSize>);

pub struct Var(i32, source::Pos);
pub struct Literal(values::Value, source::Pos);

pub struct Instruction<Source>(Instr, Source);

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
