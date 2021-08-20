use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum JvmVal {
    Byte(i8),
    Short(i16),
    Int(i32),
    Long(i64),
    String(String),
    Class(HashMap<String, JvmVal>),
    Reference(u32),
    Float(f32),
    Double(f64),
    Null,
}
