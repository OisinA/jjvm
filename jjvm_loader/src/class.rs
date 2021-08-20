use crate::const_pool::ConstPool;

#[derive(Debug, Clone)]
pub struct Class {
    pub major: u16,
    pub minor: u16,

    pub const_pool: ConstPool,
    pub name: String,
    pub superclass: String,
    pub flags: u16,
    pub interfaces: Vec<String>,
    pub fields: Vec<Field>,
    pub methods: Vec<Field>,
    pub attributes: Vec<Attribute>,
}

#[derive(Debug, Clone)]
pub struct Attribute {
    pub name: String,
    pub data: Vec<u8>,
}

#[derive(Debug, Clone)]
pub struct Field {
    pub flags: u16,
    pub name: String,
    pub descriptor: String,
    pub attributes: Vec<Attribute>,
}
