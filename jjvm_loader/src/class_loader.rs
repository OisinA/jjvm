use std::io::{Cursor, Read};

use crate::class::{Attribute, Class, Field};
use crate::const_pool::{Const, ConstPool};
use crate::jvm_const::JvmConst;

/// ClassLoader is used to load a JVM Class file
pub struct ClassLoader {
    pub bytes: Cursor<Vec<u8>>,
}

impl ClassLoader {
    pub fn new(bytes: Cursor<Vec<u8>>) -> ClassLoader {
        ClassLoader { bytes }
    }

    /// Loads the class file
    pub fn load(self: &mut ClassLoader) -> Class {
        self.u4();

        let minor = self.u2();
        let major = self.u2();

        let const_pool = self.cp_info();

        let flags = self.u2();
        let name = match const_pool.resolve(self.u2()).unwrap() {
            Const::String(val) => val,
            _ => panic!("non-string const"),
        };
        let superclass = match const_pool.resolve(self.u2()).unwrap() {
            Const::String(val) => val,
            _ => panic!("non-string const"),
        };
        let interfaces = self.load_interfaces(&const_pool);
        let fields = self.load_fields(&const_pool);
        let methods = self.load_fields(&const_pool);
        let attributes = self.load_attributes(&const_pool);

        Class {
            major,
            minor,
            const_pool,
            name,
            superclass,
            flags,
            interfaces,
            fields,
            methods,
            attributes,
        }
    }

    fn load_interfaces(self: &mut ClassLoader, const_pool: &ConstPool) -> Vec<String> {
        let interface_count = self.u2();
        let mut interfaces = vec![];
        for _ in 0..interface_count {
            interfaces.push(match const_pool.resolve(self.u2()).unwrap() {
                Const::String(val) => val,
                _ => panic!("non-string const"),
            });
        }

        interfaces
    }

    fn load_fields(self: &mut ClassLoader, const_pool: &ConstPool) -> Vec<Field> {
        let field_count = self.u2();
        let mut fields = vec![];
        for _ in 0..field_count {
            fields.push(Field {
                flags: self.u2(),
                name: match const_pool.resolve(self.u2()).unwrap() {
                    Const::String(val) => val,
                    _ => panic!("non-string const"),
                },
                descriptor: match const_pool.resolve(self.u2()).unwrap() {
                    Const::String(val) => val,
                    _ => panic!("non-string const"),
                },
                attributes: self.load_attributes(const_pool),
            })
        }

        fields
    }

    fn load_attributes(self: &mut ClassLoader, const_pool: &ConstPool) -> Vec<Attribute> {
        let attribute_count = self.u2();
        let mut attributes = vec![];

        for _ in 0..attribute_count {
            let name = self.u2();
            let data_count = self.u4();
            attributes.push(Attribute {
                name: match const_pool.resolve(name).unwrap() {
                    Const::String(val) => val,
                    _ => panic!("non-string const"),
                },
                data: self.read_bytes(data_count as i32),
            })
        }

        attributes
    }

    /// Read a single byte from the byte stream
    pub fn u1(self: &mut ClassLoader) -> u8 {
        let mut val = [0u8; 1];
        self.bytes.read_exact(&mut val).unwrap();

        u8::from_be_bytes(val)
    }

    /// Read two bytes from the byte stream
    pub fn u2(self: &mut ClassLoader) -> u16 {
        let mut val = [0u8; 2];
        self.bytes.read_exact(&mut val).unwrap();

        u16::from_be_bytes(val)
    }

    /// Read four bytes from the byte stream
    pub fn u4(self: &mut ClassLoader) -> u32 {
        let mut val = [0u8; 4];
        self.bytes.read_exact(&mut val).unwrap();

        u32::from_be_bytes(val)
    }

    /// Read four bytes from the byte stream
    pub fn u8(self: &mut ClassLoader) -> u64 {
        let mut val = [0u8; 8];
        self.bytes.read_exact(&mut val).unwrap();

        u64::from_be_bytes(val)
    }

    pub fn read_bytes(self: &mut ClassLoader, count: i32) -> Vec<u8> {
        let mut bytes = vec![];

        for _ in 0..count {
            bytes.push(self.u1());
        }

        bytes
    }

    pub fn cp_info(self: &mut ClassLoader) -> ConstPool {
        let const_pool_count = self.u2();
        let mut consts = vec![];

        for _ in 1..const_pool_count {
            let tag = self.u1();
            let result = JvmConst::from_tag(tag, self).unwrap();
            consts.push(result);
        }

        ConstPool { consts }
    }
}
