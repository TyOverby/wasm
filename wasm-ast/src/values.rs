pub enum Values<A, B, C, D> {
    I32(A),
    I64(B),
    F32(C),
    F64(D),
}

pub type Value = Values<i32, i64, f32, f64>;
