use crate::jvm_const::JvmConst;

#[derive(Debug, Clone)]
pub struct ConstPool {
    pub consts: Vec<JvmConst>,
}

#[derive(Debug, Clone)]
pub enum Const {
    String(String),
    FieldRef(Box<Const>, Box<Const>),
    NameAndType(Box<Const>, Box<Const>),
    MethodRef(Box<Const>, Box<Const>),
    Integer(i32),
    Float(f32),
}

impl ConstPool {
    pub fn resolve(self: &ConstPool, index: u16) -> Result<Const, String> {
        match &self.consts[index as usize - 1] {
            JvmConst::UTF8(val) => Ok(Const::String(val.clone())),
            JvmConst::String(i) => self.resolve(*i),
            JvmConst::Integer(i) => Ok(Const::Integer(*i as i32)),
            JvmConst::Float(f) => Ok(Const::Float(f32::from_be_bytes(f.to_be_bytes()))),
            JvmConst::Class(c) => self.resolve(*c),
            JvmConst::FieldRef(i, j) => Ok(Const::FieldRef(
                Box::new(self.resolve(*i).unwrap()),
                Box::new(self.resolve(*j).unwrap()),
            )),
            JvmConst::NameAndType(i, j) => Ok(Const::NameAndType(
                Box::new(self.resolve(*i).unwrap()),
                Box::new(self.resolve(*j).unwrap()),
            )),
            JvmConst::MethodRef(i, j) => Ok(Const::MethodRef(
                Box::new(self.resolve(*i).unwrap()),
                Box::new(self.resolve(*j).unwrap()),
            )),
            _ => Err("const not found".to_string()),
        }
    }
}
