use crate::class_loader::ClassLoader;

#[derive(Debug, Clone)]
pub enum JvmConst {
    UTF8(String),
    Integer(u32),
    Float(u32),
    Long(u32, u32),
    Double(u32, u32),
    Class(u16),
    String(u16),
    FieldRef(u16, u16),
    MethodRef(u16, u16),
    InterfaceMethodRef(u16, u16),
    NameAndType(u16, u16),
    MethodHandle(u8, u8),
    MethodType(u16),
    InvokeDynamic(u16, u16),
}

impl JvmConst {
    pub fn from_tag(tag: u8, loader: &mut ClassLoader) -> Result<JvmConst, String> {
        match tag {
            0x01 => {
                let string_length = loader.u2();
                Ok(JvmConst::UTF8(
                    cesu8::from_java_cesu8(&loader.read_bytes(string_length as i32))
                        .unwrap()
                        .to_string(),
                ))
            }
            0x03 => Ok(JvmConst::Integer(loader.u4())),
            0x04 => Ok(JvmConst::Float(loader.u4())),
            0x05 => Ok(JvmConst::Long(loader.u4(), loader.u4())),
            0x06 => Ok(JvmConst::Double(loader.u4(), loader.u4())),
            0x07 => Ok(JvmConst::Class(loader.u2())),
            0x08 => Ok(JvmConst::String(loader.u2())),
            0x09 => Ok(JvmConst::FieldRef(loader.u2(), loader.u2())),
            0x0a => Ok(JvmConst::MethodRef(loader.u2(), loader.u2())),
            0x0b => Ok(JvmConst::InterfaceMethodRef(loader.u2(), loader.u2())),
            0x0c => Ok(JvmConst::NameAndType(loader.u2(), loader.u2())),
            0x0f => Ok(JvmConst::MethodHandle(loader.u1(), loader.u1())),
            0x10 => Ok(JvmConst::MethodType(loader.u2())),
            0x11 => Ok(JvmConst::InvokeDynamic(loader.u2(), loader.u2())),
            _ => Err(format!("invalid tag {:#04x}", tag)),
        }
    }
}
