use std::{collections::HashMap, time::Instant};

use jjvm_loader::{class::Class, const_pool::Const, flags::MethodFlag, opcode::Opcode, signature};

use crate::{
    frame::Frame,
    heap::Heap,
    jvm_val::JvmVal,
    stdlib::{self, class::BuiltinClass},
};
use logging_timer::{time, timer};

pub struct VM {
    pub heap: Heap,
    pub classes: HashMap<String, Class>,
    pub references: HashMap<i32, Vec<u32>>,

    pub heap_last_gc_size: usize,
    pub should_gc: bool,

    pub debug: bool,
}

impl VM {
    #[time]
    pub fn exec(self: &mut VM, class: &Class, frame: &mut Frame) -> JvmVal {
        while frame.ip < frame.code.len() as u32 {
            let op = frame.code[frame.ip as usize];
            self.debug(
                frame.id,
                format!(
                    "CodeIndex: {:?} Opcode: {:?} Stack: {:?}",
                    frame.ip,
                    Opcode::from(op),
                    frame.stack
                ),
            );

            if self.should_gc || self.heap.allocated_items() >= self.heap_last_gc_size * 2 {
                let start = Instant::now();
                let mut refs = vec![];
                for i in &frame.stack {
                    if let JvmVal::Reference(x) = i {
                        refs.push(*x);
                    }
                }
                for i in &frame.locals {
                    if let JvmVal::Reference(x) = i {
                        refs.push(*x);
                    }
                }
                self.references.insert(frame.id, refs.clone());
                let claimed = self.heap.gc(self.references.clone());
                self.heap_last_gc_size = self.heap.allocated_items();
                self.debug(frame.id, format!("Heap size: {:?}", self.heap.heap.len()));
                self.debug(frame.id, format!("Reclaimed blocks: {}", claimed));
                let duration = start.elapsed();
                self.debug(frame.id, format!("GC ran in {}ms", duration.as_millis()));
                self.should_gc = false;
            }

            let _l = timer!("Matching", "Opcode = {:?}", Opcode::from(op));
            match Opcode::from(op) {
                Opcode::Nop => {}
                Opcode::IConstM1 => frame.stack.push(JvmVal::Int(-1)),
                Opcode::IConst0 => frame.stack.push(JvmVal::Int(0)),
                Opcode::IConst1 => frame.stack.push(JvmVal::Int(1)),
                Opcode::IConst2 => frame.stack.push(JvmVal::Int(2)),
                Opcode::IConst3 => frame.stack.push(JvmVal::Int(3)),
                Opcode::IConst4 => frame.stack.push(JvmVal::Int(4)),
                Opcode::IConst5 => frame.stack.push(JvmVal::Int(5)),
                Opcode::AConstNull => frame.stack.push(JvmVal::Null),
                Opcode::ILoad => {
                    let index = frame.read_one_byte_index();
                    frame.stack.push(frame.locals[index as usize].clone());
                }
                Opcode::ILoad0 => frame.stack.push(frame.locals[0].clone()),
                Opcode::ILoad1 => frame.stack.push(frame.locals[1].clone()),
                Opcode::ILoad2 => frame.stack.push(frame.locals[2].clone()),
                Opcode::ILoad3 => frame.stack.push(frame.locals[3].clone()),
                Opcode::FLoad0 => frame.stack.push(frame.locals[0].clone()),
                Opcode::FLoad1 => frame.stack.push(frame.locals[1].clone()),
                Opcode::FLoad2 => frame.stack.push(frame.locals[2].clone()),
                Opcode::FLoad3 => frame.stack.push(frame.locals[3].clone()),
                Opcode::ALoad0 => {
                    let refer = frame.locals[0].clone();
                    frame.stack.push(refer)
                }
                Opcode::ALoad1 => {
                    let refer = frame.locals[1].clone();
                    frame.stack.push(refer)
                }
                Opcode::ALoad2 => {
                    let refer = frame.locals[2].clone();
                    frame.stack.push(refer)
                }
                Opcode::ALoad3 => {
                    let refer = frame.locals[3].clone();
                    frame.stack.push(refer)
                }
                Opcode::ALoad => {
                    let index = frame.read_one_byte_index();
                    frame.stack.push(frame.locals[index as usize].clone());
                }
                Opcode::AaLoad => {
                    let index = frame.pop_int();
                    let arrayref = match frame.stack.pop().unwrap() {
                        JvmVal::Reference(x) => x,
                        _ => panic!("ALoad: Expected reference"),
                    };

                    let array = match self.heap.fetch(arrayref) {
                        JvmVal::Array(vals) => &vals[index as usize],
                        _ => panic!("ALoad: Expected array"),
                    };

                    let refer = match array {
                        JvmVal::Reference(x) => *x,
                        _ => panic!("ALoad: Expected reference"),
                    };

                    let loaded = self.heap.fetch(refer);

                    frame.stack.push(loaded.clone());
                }
                Opcode::IStore0 => frame.locals[0] = JvmVal::Int(frame.pop_int()),
                Opcode::IStore1 => frame.locals[1] = JvmVal::Int(frame.pop_int()),
                Opcode::IStore2 => frame.locals[2] = JvmVal::Int(frame.pop_int()),
                Opcode::IStore3 => frame.locals[3] = JvmVal::Int(frame.pop_int()),
                Opcode::IStore => {
                    let _istore = timer!("ISTORE");
                    let index = frame.read_one_byte_index();
                    if frame.locals.len() as u8 <= index {
                        frame
                            .locals
                            .extend(vec![JvmVal::Null; index as usize - frame.locals.len() + 1]);
                    }

                    frame.locals[index as usize] = frame.stack.pop().unwrap();
                }
                Opcode::FStore0 => frame.locals[0] = JvmVal::Float(frame.pop_float()),
                Opcode::FStore1 => frame.locals[1] = JvmVal::Float(frame.pop_float()),
                Opcode::FStore2 => frame.locals[2] = JvmVal::Float(frame.pop_float()),
                Opcode::FStore3 => frame.locals[3] = JvmVal::Float(frame.pop_float()),
                Opcode::FStore => {
                    let _istore = timer!("ISTORE");
                    let index = frame.read_one_byte_index();
                    if frame.locals.len() as u8 <= index {
                        frame
                            .locals
                            .extend(vec![JvmVal::Null; index as usize - frame.locals.len() + 1]);
                    }

                    frame.locals[index as usize] = frame.stack.pop().unwrap();
                }
                Opcode::AStore => {
                    let _istore = timer!("ASTORE");
                    let index = frame.read_one_byte_index();
                    if frame.locals.len() as u8 <= index {
                        frame
                            .locals
                            .extend(vec![JvmVal::Null; index as usize - frame.locals.len() + 1]);
                    }

                    frame.locals[index as usize] = frame.stack.pop().unwrap();
                }
                Opcode::AStore0 => frame.locals[0] = frame.stack.pop().unwrap(),
                Opcode::AStore1 => frame.locals[1] = frame.stack.pop().unwrap(),
                Opcode::AStore2 => frame.locals[2] = frame.stack.pop().unwrap(),
                Opcode::AStore3 => frame.locals[3] = frame.stack.pop().unwrap(),
                Opcode::IAdd => {
                    let b = frame.pop_int();
                    let a = frame.pop_int();
                    frame.push(JvmVal::Int(a + b));
                }
                Opcode::ISub => {
                    let b = frame.pop_int();
                    let a = frame.pop_int();
                    frame.push(JvmVal::Int(a - b));
                }
                Opcode::IMul => {
                    let b = frame.pop_int();
                    let a = frame.pop_int();
                    frame.push(JvmVal::Int(a * b));
                }
                Opcode::IRem => {
                    let b = frame.pop_int();
                    let a = frame.pop_int();
                    frame.push(JvmVal::Int(a - (a / b) * b));
                }
                Opcode::FAdd => {
                    let b = frame.pop_float();
                    let a = frame.pop_float();
                    frame.push(JvmVal::Float(a + b));
                }
                Opcode::FSub => {
                    let b = frame.pop_float();
                    let a = frame.pop_float();
                    frame.push(JvmVal::Float(a - b));
                }
                Opcode::FMul => {
                    let b = frame.pop_float();
                    let a = frame.pop_float();
                    frame.push(JvmVal::Float(a * b));
                }
                Opcode::FRem => {
                    let b = frame.pop_float();
                    let a = frame.pop_float();
                    frame.push(JvmVal::Float(a - (a / b) * b));
                }
                Opcode::IInc => {
                    let index = frame.read_one_byte_index();
                    let cons = frame.read_one_byte_index();

                    match frame.locals[index as usize] {
                        JvmVal::Int(v) => {
                            frame.locals[index as usize] = JvmVal::Int(v + cons as i32)
                        }
                        _ => panic!("not an int"),
                    };
                }
                Opcode::Goto => {
                    let offset = frame.read_two_byte_index() as i16;
                    frame.ip = (frame.ip as i16 + offset - 3) as u32;
                    // self.should_gc = true;
                }
                Opcode::IfIcmpNe => {
                    let offset = frame.read_two_byte_index();
                    let b = frame.pop_int();
                    let a = frame.pop_int();
                    if a != b {
                        frame.ip = (frame.ip as i32 + offset as i32 - 3) as u32;
                    }
                }
                Opcode::IfIcmpGe => {
                    let offset = frame.read_two_byte_index();
                    let b = frame.pop_int();
                    let a = frame.pop_int();
                    if a >= b {
                        frame.ip = (frame.ip as i32 + offset as i32 - 3) as u32;
                    }
                }
                Opcode::IfIcmpGt => {
                    let offset = frame.read_two_byte_index();
                    let b = frame.pop_int();
                    let a = frame.pop_int();
                    if a > b {
                        frame.ip = (frame.ip as i32 + offset as i32 - 3) as u32
                    }
                }
                Opcode::IfIcmpLe => {
                    let offset = frame.read_two_byte_index();
                    let b = frame.pop_int();
                    let a = frame.pop_int();
                    if a <= b {
                        frame.ip = (frame.ip as i32 + offset as i32 - 3) as u32;
                    }
                }
                Opcode::IfNe => {
                    let offset = frame.read_two_byte_index();
                    let b = frame.pop_int();
                    if b != 0 {
                        frame.ip = (frame.ip as i32 + offset as i32 - 3) as u32
                    }
                }
                Opcode::IfEq => {
                    let offset = frame.read_two_byte_index();
                    let b = frame.pop_int();
                    if b == 0 {
                        frame.ip = (frame.ip as i32 + offset as i32 - 3) as u32
                    }
                }
                Opcode::IfNonNull => {
                    let offset = frame.read_two_byte_index();
                    let popped = frame.stack.pop().unwrap();
                    // let ptr = match popped {
                    //     JvmVal::Int(i) => i,
                    //     _ => panic!("not an int, got {:?}", popped),
                    // };
                    // let b = self.heap.fetch(ptr as u32);
                    if popped != JvmVal::Null {
                        frame.ip = (frame.ip as i32 + offset as i32 - 3) as u32
                    }
                }
                Opcode::IReturn => {
                    self.references.remove(&frame.id);
                    return JvmVal::Int(frame.pop_int());
                }
                Opcode::FReturn => {
                    self.references.remove(&frame.id);
                    return JvmVal::Float(frame.pop_float());
                }
                Opcode::AReturn => {
                    self.references.remove(&frame.id);
                    return frame.stack.pop().unwrap();
                }
                Opcode::Return => {
                    self.references.remove(&frame.id);
                    return JvmVal::Null;
                }
                Opcode::Ldc => {
                    let _l = timer!("LDC");
                    let index = frame.read_one_byte_index();
                    let val = class.const_pool.resolve(index as u16).unwrap();

                    frame.push(match val {
                        Const::String(v) => JvmVal::String(v),
                        Const::Integer(v) => JvmVal::Int(v),
                        Const::Float(v) => JvmVal::Float(v),
                        _ => panic!("non-string constant"),
                    })
                }
                Opcode::InvokeVirtual => {
                    let index = frame.read_two_byte_index();
                    let args = vec![];
                    let result = self.invoke_virtual(class, frame, index, args);
                    if let JvmVal::Null = result {
                    } else {
                        frame.push(result);
                    }
                }
                Opcode::GetStatic => {
                    let index = frame.read_two_byte_index();
                    self.get_static(class, frame, index);
                }
                Opcode::InvokeStatic => {
                    let index = frame.read_two_byte_index();
                    // let args = vec![frame.stack.pop().unwrap(), frame.stack.pop().unwrap()];
                    let result = self.invoke_static(class, frame, index, vec![]);

                    if let JvmVal::Null = result {
                    } else {
                        frame.push(result);
                    }
                }
                Opcode::InvokeSpecial => {
                    let index = frame.read_two_byte_index();
                    let result = self.invoke_special(class, frame, index, vec![]);

                    if let JvmVal::Null = result {
                    } else {
                        frame.push(result);
                    }
                }
                Opcode::BiPush => {
                    let val = frame.read_one_byte_index() as i32;
                    frame.push(JvmVal::Int(val));
                }
                Opcode::SiPush => {
                    let val = frame.read_two_byte_index() as i32;
                    frame.push(JvmVal::Int(val));
                }
                Opcode::New => {
                    let _l = timer!("New");
                    let index = frame.read_two_byte_index();
                    let cons = class.const_pool.resolve(index).unwrap();
                    let clss = self.classes.get(&match cons.clone() {
                        Const::String(val) => val,
                        _ => panic!(),
                    });

                    if clss.is_some() {
                        let cls = clss.unwrap();
                        let mut vals = HashMap::new();
                        for f in &cls.fields {
                            vals.insert(f.name.clone(), JvmVal::Int(0));
                        }

                        let ptr = self.heap.alloc(JvmVal::Class(cls.name.clone(), vals));

                        frame.push(JvmVal::Reference(ptr));
                    } else {
                        let name = match cons.clone() {
                            Const::String(val) => val,
                            _ => panic!(),
                        };
                        let builtin_cls = stdlib::get_builtins(match cons.clone() {
                            Const::String(val) => val,
                            _ => panic!(),
                        });

                        let mut vals = HashMap::new();
                        for f in builtin_cls.get_fields() {
                            vals.insert(f.name.clone(), JvmVal::Int(0));
                        }

                        let ptr = self
                            .heap
                            .alloc(JvmVal::Class(name.clone().to_string(), vals));

                        frame.push(JvmVal::Reference(ptr));
                    }
                }
                Opcode::Dup => {
                    let val = frame.stack.last().unwrap().clone();
                    frame.push(val);
                }
                Opcode::PutField => {
                    let _l = timer!("PUT_FIELD");
                    let index = frame.read_two_byte_index();
                    let v = class.const_pool.resolve(index).unwrap();
                    let (_, name, _) = deref_field_ref(v);

                    let value = frame.stack.pop().unwrap();

                    let refer = frame.stack.pop().unwrap();
                    let ptr = match refer {
                        JvmVal::Reference(ptr) => ptr,
                        _ => panic!("not a reference, got {:?}", refer),
                    };

                    match self.heap.fetch_mut(ptr) {
                        JvmVal::Class(_, vals) => vals.insert(name, value),
                        _ => panic!("not a class, got {:?}", ptr),
                    };
                }
                Opcode::GetField => {
                    let _l = timer!("GET_FIELD");
                    let index = frame.read_two_byte_index();
                    let refer = class.const_pool.resolve(index).unwrap();
                    let (_, name, _) = deref_field_ref(refer);

                    let popped = frame.stack.pop().unwrap();
                    let ptr = match popped {
                        JvmVal::Reference(ptr) => ptr,
                        JvmVal::Null => {
                            frame.stack.push(JvmVal::Null);
                            return JvmVal::Null;
                        }
                        _ => panic!("not a reference, got {:?}", popped),
                    };

                    match self.heap.fetch(ptr) {
                        JvmVal::Class(_, vals) => {
                            frame.stack.push(vals.get(&name).unwrap().clone());
                        }
                        _ => panic!("not a class at address {}", ptr),
                    }
                }
                Opcode::Pop => {
                    frame.stack.pop().unwrap();
                }
                Opcode::TableSwitch => {
                    let origin_ip = frame.ip;

                    let offset = (frame.ip + 1) % 4;
                    frame.ip += (4 - offset) % 4;

                    let default = frame.read_four_byte_index() as i32;
                    let low = frame.read_four_byte_index() as i32;
                    let high = frame.read_four_byte_index() as i32;

                    let mut offsets = vec![];
                    for _ in 0..high - low + 1 {
                        offsets.push(frame.read_four_byte_index() as i32);
                    }

                    let index = frame.pop_int();

                    if index < low || index > high {
                        frame.ip = (origin_ip as i32 + default) as u32 - 1;
                    } else {
                        let position = index - low;
                        let offset = offsets[position as usize];
                        frame.ip = ((origin_ip as i32) + offset) as u32 - 1;
                    }
                }
                Opcode::LookupSwitch => {
                    let origin_ip = frame.ip;

                    let offset = (frame.ip + 1) % 4;
                    frame.ip += (4 - offset) % 4;

                    let default = frame.read_four_byte_index() as i32;
                    let npairs = frame.read_four_byte_index() as i32;

                    let mut pairs = HashMap::new();
                    for _ in 0..npairs {
                        let int_match = frame.read_four_byte_index() as i32;
                        let offset = frame.read_four_byte_index() as i32;
                        pairs.insert(int_match, offset);
                    }

                    let key = frame.pop_int();

                    if !pairs.contains_key(&key) {
                        frame.ip = (origin_ip as i32 + default) as u32 - 1;
                    } else {
                        let offset = pairs.get(&key).unwrap();
                        frame.ip = ((origin_ip as i32) + offset) as u32 - 1;
                    }
                }
                Opcode::InstanceOf => {
                    let _ = frame.read_two_byte_index();

                    frame.stack.push(JvmVal::Int(1));
                }
                Opcode::CheckCast => {
                    let _ = frame.read_two_byte_index();
                }
                _ => panic!("unhandled opcode {:?}, {:#04x}", Opcode::from(op), op),
            }

            frame.ip += 1;
        }

        self.references.remove(&frame.id);
        JvmVal::Null
    }

    #[time]
    fn invoke_virtual(
        self: &mut VM,
        class: &Class,
        frame: &mut Frame,
        index: u16,
        _args: Vec<JvmVal>,
    ) -> JvmVal {
        let l = class.const_pool.resolve(index).unwrap();
        self.debug(frame.id, format!("invoke_virtual: {:?}", l));
        match l {
            Const::MethodRef(i, l) => match *i {
                Const::String(val) => match val.as_str() {
                    // Cheat-y way of getting System.out.println working without a standard library to access
                    "java/io/PrintStream" => {
                        let args = vec![frame.stack.pop().unwrap()];
                        match &args[0] {
                            JvmVal::String(val) => println!("{}", val),
                            JvmVal::Int(val) => println!("{}", val),
                            JvmVal::Float(val) => println!("{}", val),
                            JvmVal::Reference(val) => {
                                let val = self.heap.fetch(*val);
                                match val {
                                    JvmVal::Class(name, vals) => {
                                        if name == "java/lang/Boolean" {
                                            println!(
                                                "{}",
                                                if vals.get("value").unwrap().clone()
                                                    == JvmVal::Int(0)
                                                {
                                                    "false"
                                                } else {
                                                    "true"
                                                }
                                            );
                                        } else {
                                            for (k, v) in vals {
                                                println!("{:?}: {:?}", k, v);
                                            }
                                        }
                                    }
                                    _ => panic!("not a class"),
                                }
                            }
                            _ => println!("{:?}", args[0]),
                        }
                    }
                    _ => {
                        let mut args = vec![];
                        let (name, typ) = match *l {
                            Const::NameAndType(name, typ) => match *name {
                                Const::String(val) => match *typ {
                                    Const::String(v) => (val, v),
                                    _ => panic!(),
                                },
                                _ => panic!(),
                            },
                            _ => panic!(),
                        };
                        for _ in 0..parse_descriptors(typ) {
                            args.push(frame.stack.pop().unwrap());
                        }
                        let clss = self.classes.get(&val);
                        if clss.is_some() {
                            let cls = clss.unwrap().clone();
                            if !MethodFlag::Static
                                .is_set(cls.methods.iter().find(|x| x.name == name).unwrap().flags)
                            {
                                let refer = frame.stack.pop().unwrap().clone();
                                args.insert(0, refer);
                            }
                            let mut f = Frame::from_method(&cls, name, args).unwrap();

                            let result = self.exec(&cls, &mut f);
                            frame.stack.push(result);
                        } else {
                            let builtin = stdlib::get_builtins(val.clone());

                            let refer = frame.stack.pop().unwrap().clone();
                            args.insert(0, refer);

                            return builtin.get_method(name)(self, args);
                        }
                    }
                },
                _ => panic!(),
            },
            _ => panic!(),
        }
        JvmVal::Null
    }

    fn get_static(self: &mut VM, class: &Class, _: &mut Frame, index: u16) {
        let _ = class.const_pool.resolve(index);

        // Actually load static
    }

    #[time]
    fn invoke_static(
        self: &mut VM,
        class: &Class,
        frame: &mut Frame,
        index: u16,
        _: Vec<JvmVal>,
    ) -> JvmVal {
        let v = class.const_pool.resolve(index).unwrap();
        self.debug(frame.id, format!("invoke_static: {:?}", v));
        let (class_name, name, typ) = match v {
            Const::MethodRef(class_name, nat) => match *nat {
                Const::NameAndType(name, typ) => (
                    match *class_name {
                        Const::String(v) => v,
                        _ => panic!(),
                    },
                    match *name {
                        Const::String(v) => v,
                        _ => panic!(),
                    },
                    match *typ {
                        Const::String(v) => v,
                        _ => panic!(),
                    },
                ),
                _ => panic!(""),
            },
            _ => panic!(),
        };

        let mut args = vec![];
        for _ in 0..parse_descriptors(typ) {
            args.push(frame.stack.pop().unwrap());
        }

        let clss = self.classes.get(&class_name);

        if clss.is_some() {
            let cls = clss.unwrap();

            let mut f = Frame::from_method(cls, name, args).unwrap();

            let _a = timer!("Static Exec");
            return self.exec(class, &mut f);
        }

        let builtin = stdlib::get_builtins(class_name.clone());

        return builtin.get_method(name)(self, args);
    }

    #[time]
    fn invoke_special(
        self: &mut VM,
        class: &Class,
        frame: &mut Frame,
        index: u16,
        _: Vec<JvmVal>,
    ) -> JvmVal {
        let l = class.const_pool.resolve(index).unwrap();
        self.debug(frame.id, format!("invoke_special: {:?}", l));
        match l {
            Const::MethodRef(i, l) => match *i {
                Const::String(val) => {
                    let (name, typ) = match *l {
                        Const::NameAndType(name, typ) => match *name {
                            Const::String(val) => (
                                val,
                                match *typ {
                                    Const::String(t) => t,
                                    _ => panic!(),
                                },
                            ),
                            _ => panic!(),
                        },
                        _ => panic!(),
                    };
                    let mut args = vec![];
                    for _ in 0..parse_descriptors(typ) {
                        args.push(frame.stack.pop().unwrap());
                    }
                    args.reverse();
                    args.insert(0, frame.stack.pop().unwrap());

                    if val != *"java/lang/Object" {
                        // let cls = self.classes.get(&val).unwrap().clone();
                        let clss = self.classes.get(&val);
                        if clss.is_some() {
                            let cls = clss.unwrap().clone();
                            let mut f = Frame::from_method(&cls, name, args).unwrap();
                            let _a = timer!("Special Exec");
                            let result = self.exec(&cls, &mut f);
                            return result;
                        }

                        let builtin = stdlib::get_builtins(val.clone());

                        return builtin.get_method(name)(self, args);
                    }
                }
                _ => panic!(),
            },
            _ => {}
        }
        JvmVal::Null
    }

    pub fn debug(self: &mut VM, frame_id: i32, message: String) {
        if !self.debug {
            return;
        }
        println!("{}: {}", frame_id, message);
    }
}

#[time]
fn parse_descriptors(descriptor: String) -> usize {
    signature::TypeSignature::from_str(descriptor)
        .unwrap()
        .args
        .len()
}

#[time]
fn deref_field_ref(cn: Const) -> (String, String, String) {
    match cn {
        Const::FieldRef(i, l) => match *i {
            Const::String(val1) => match *l {
                Const::NameAndType(name, typ) => match *name {
                    Const::String(val) => match *typ {
                        Const::String(t) => (val1, val, t),
                        _ => panic!(),
                    },
                    _ => panic!(),
                },
                _ => panic!(),
            },
            _ => panic!(),
        },
        // Const::String(val) => ("".to_string(), val, "".to_string()),
        _ => panic!("not a fieldref, got {:?}", cn),
    }
}
