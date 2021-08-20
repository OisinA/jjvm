use std::{
    fs,
    io::{Cursor, Read},
};

use jvm_const::JvmConst;

pub mod jvm_const;

#[derive(Debug, Clone)]
struct ConstPool {
    pub consts: Vec<JvmConst>,
}

impl ConstPool {
    fn resolve(self: &ConstPool, index: u16) -> String {
        if let JvmConst::String(val) = &self.consts[(index - 1) as usize] {
            return val.clone();
        }

        "".to_string()
    }
}

#[derive(Debug, Clone)]
struct Loader {
    pub bytes: Cursor<Vec<u8>>,
}

impl Loader {
    fn u1(self: &mut Loader) -> u8 {
        let mut val = [0u8; 1];
        self.bytes.read_exact(&mut val).unwrap();

        u8::from_be_bytes(val)
    }

    fn u2(self: &mut Loader) -> u16 {
        let mut val = [0u8; 2];
        self.bytes.read_exact(&mut val).unwrap();

        u16::from_be_bytes(val)
    }

    fn u4(self: &mut Loader) -> u32 {
        let mut val = [0u8; 4];
        self.bytes.read_exact(&mut val).unwrap();

        u32::from_be_bytes(val)
    }

    fn u8(self: &mut Loader) -> u64 {
        let mut val = [0u8; 8];
        self.bytes.read_exact(&mut val).unwrap();

        u64::from_be_bytes(val)
    }

    fn read_bytes(self: &mut Loader, count: i32) -> Vec<u8> {
        let mut bytes = vec![];

        for _ in 0..count {
            bytes.push(self.u1());
        }

        bytes
    }

    fn cp_info(self: &mut Loader) -> Vec<JvmConst> {
        let const_pool_count = self.u2();
        let mut consts = vec![];

        for _ in 1..const_pool_count {
            let tag = self.u1();
            consts.push(match tag {
                0x01 => {
                    let string_length = self.u2();
                    JvmConst::String(
                        std::str::from_utf8(&self.read_bytes(string_length as i32))
                            .unwrap()
                            .to_string(),
                    )
                }
                0x07 => JvmConst::NameIndex(self.u2()),
                0x08 => JvmConst::StringIndex(self.u2()),
                0x09 | 0x0a => JvmConst::ClassIndex(self.u2(), self.u2()),
                0x0c => JvmConst::NameAndDesc(self.u2(), self.u2()),
                _ => panic!("unsupported tag: {}", tag),
            });
        }

        consts
    }

    fn load_interfaces(self: &mut Loader, consts: &ConstPool) -> Vec<String> {
        let interface_count = self.u2();
        let mut interfaces = vec![];
        for _ in 0..interface_count {
            interfaces.push(consts.resolve(self.u2()));
        }

        interfaces
    }

    fn load_fields(self: &mut Loader, consts: &ConstPool) -> Vec<Field> {
        let field_count = self.u2();
        let mut fields = vec![];
        for _ in 0..field_count {
            fields.push(Field {
                flags: self.u2(),
                name: consts.resolve(self.u2()),
                descriptor: consts.resolve(self.u2()),
                attributes: self.load_attributes(consts),
            })
        }

        fields
    }

    fn load_attributes(self: &mut Loader, consts: &ConstPool) -> Vec<Attribute> {
        let attribute_count = self.u2();
        let mut attributes = vec![];

        for _ in 0..attribute_count {
            let name = self.u2();
            let data_count = self.u4();
            attributes.push(Attribute {
                name: consts.resolve(name),
                data: self.read_bytes(data_count as i32),
            })
        }

        attributes
    }
}

#[derive(Debug, Clone)]
struct Field {
    pub flags: u16,
    pub name: String,
    pub descriptor: String,
    pub attributes: Vec<Attribute>,
}

#[derive(Debug, Clone)]
struct Attribute {
    pub name: String,
    pub data: Vec<u8>,
}

#[derive(Debug, Clone)]
struct Class {
    consts: ConstPool,
    name: String,
    superclass: String,
    flags: u16,
    interfaces: Vec<String>,
    fields: Vec<Field>,
    methods: Vec<Field>,
    attributes: Vec<Attribute>,
}

impl Class {
    pub fn new(loader: &mut Loader) -> Class {
        loader.u4();

        let minor = loader.u2();
        let major = loader.u2();

        println!("Loading class v {} {}", major, minor);

        let cp = ConstPool {
            consts: loader.cp_info(),
        };

        let flags = loader.u2();
        let name = cp.resolve(loader.u2());
        let superclass = cp.resolve(loader.u2());
        let interfaces = loader.load_interfaces(&cp);
        let fields = loader.load_fields(&cp);
        let methods = loader.load_fields(&cp);
        let attributes = loader.load_attributes(&cp);

        Class {
            consts: cp,
            name,
            superclass,
            flags,
            interfaces,
            fields,
            methods,
            attributes,
        }
    }

    fn frame(self: Class, method: String, args: Vec<i32>) -> Frame {
        let m = self
            .methods
            .iter()
            .filter(|item| item.name == method)
            .next()
            .unwrap();

        let a = m
            .attributes
            .iter()
            .filter(|att| att.name == "Code" && att.data.len() > 8)
            .next()
            .unwrap();

        // let max_locals = u16::from_be_bytes(a.data[2..4].try_into().unwrap());
        let mut frame = Frame {
            code: a.data[8..].to_vec(),
            locals: vec![],
            ip: 0,
            stack: vec![],
        };

        for i in 0..args.len() {
            frame.locals.push(args[i]);
        }

        frame
    }
}

struct Frame {
    ip: u32,
    code: Vec<u8>,
    locals: Vec<i32>,
    stack: Vec<i32>,
}

impl Frame {
    fn exec(self: &mut Frame) -> i32 {
        loop {
            let op = self.code[self.ip as usize];
            println!("Op: {:?} Stack: {:?}", op, self.stack);

            match op {
                26 => self.stack.push(self.locals[0]),
                27 => self.stack.push(self.locals[1]),
                96 => {
                    let a = self.stack.pop().unwrap();
                    let b = self.stack.pop().unwrap();
                    self.stack.push(a + b);
                }
                172 => {
                    let val = self.stack.pop().unwrap();
                    return val;
                }
                _ => panic!("unknown opcode {}", op),
            }

            self.ip += 1;
        }
    }
}

fn main() {
    let mut loader = Loader {
        bytes: Cursor::new(fs::read("Add.class").unwrap()),
    };

    let class = Class::new(&mut loader);
    println!("Calling {} function with args [2, 3]", "add");
    let mut frame = class.frame("add".to_string(), vec![200, 33]);
    let result = frame.exec();
    println!("{:?}", result);
}
