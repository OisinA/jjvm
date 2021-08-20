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
        if let JvmConst::UTF8(val) = &self.consts[(index - 1) as usize] {
            return Ok(Const::String(val.clone()));
        }

        if let JvmConst::String(i) = &self.consts[(index - 1) as usize] {
            return self.resolve(*i);
        }

        if let JvmConst::Integer(i) = &self.consts[(index - 1) as usize] {
            return Ok(Const::Integer(*i as i32));
        }

        if let JvmConst::Float(i) = &self.consts[(index - 1) as usize] {
            return Ok(Const::Float(f32::from_be_bytes(i.to_be_bytes())));
        }

        if let JvmConst::Class(i) = &self.consts[(index - 1) as usize] {
            return self.resolve(*i);
        }

        if let JvmConst::FieldRef(i, j) = &self.consts[(index - 1) as usize] {
            return Ok(Const::FieldRef(
                Box::new(self.resolve(*i).unwrap()),
                Box::new(self.resolve(*j).unwrap()),
            ));
        }

        if let JvmConst::NameAndType(name, typ) = &self.consts[(index - 1) as usize] {
            return Ok(Const::NameAndType(
                Box::new(self.resolve(*name).unwrap()),
                Box::new(self.resolve(*typ).unwrap()),
            ));
        }

        if let JvmConst::MethodRef(x, y) = &self.consts[(index - 1) as usize] {
            return Ok(Const::MethodRef(
                Box::new(self.resolve(*x).unwrap()),
                Box::new(self.resolve(*y).unwrap()),
            ));
        }

        Err("const not found".to_string())
    }
}
