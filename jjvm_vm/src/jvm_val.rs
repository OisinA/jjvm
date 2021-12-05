use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum JvmVal {
    Byte(i8),
    Short(i16),
    Int(i32),
    Long(i64),
    String(String),
    Class(String, HashMap<String, JvmVal>),
    BuiltinClass(String, HashMap<String, JvmVal>),
    Reference(u32),
    Float(f32),
    Double(f64),
    Null,
    Boolean(bool),
    Array(Vec<JvmVal>),
}
