use core::panic;

use jjvm_loader::class::Class;

use crate::jvm_val::JvmVal;
use logging_timer::time;
use std::convert::TryInto;

pub struct Frame {
    pub id: i32,
    pub ip: u32,
    pub code: Vec<u8>,
    pub locals: Vec<JvmVal>,
    pub stack: Vec<JvmVal>,
}

static mut FRAME_ID: i32 = 0;

impl Frame {
    #[time]
    pub fn from_method(class: &Class, method: String, args: Vec<JvmVal>) -> Result<Frame, String> {
        let m = class.methods.iter().find(|item| item.name == method);
        if m.is_none() {
            return Err(format!("method not found {}.{}", class.name, method));
        }

        let code_attribute = m
            .unwrap()
            .attributes
            .iter()
            .find(|attr| attr.name == "Code" && attr.data.len() > 8)
            .unwrap();

        let mut frame = Frame {
            id: unsafe { FRAME_ID },
            code: code_attribute.data[8..].to_vec(),
            locals: vec![
                JvmVal::Null,
                JvmVal::Null,
                JvmVal::Null,
                JvmVal::Null,
                JvmVal::Null,
            ],
            ip: 0,
            stack: vec![],
        };

        unsafe { FRAME_ID += 1 };

        for (i, arg) in args.iter().enumerate() {
            frame.locals[i] = arg.clone();
        }

        Ok(frame)
    }

    pub fn pop_int(self: &mut Frame) -> i32 {
        let val = self.stack.pop().unwrap();
        if let JvmVal::Int(x) = val {
            return x;
        }

        panic!("popped value was not int, got {:?}", val);
    }

    pub fn pop_float(self: &mut Frame) -> f32 {
        let val = self.stack.pop().unwrap();
        if let JvmVal::Float(x) = val {
            return x;
        }

        panic!("popped value was not float, got {:?}", val);
    }

    pub fn pop_double(self: &mut Frame) -> f64 {
        let val = self.stack.pop().unwrap();
        if let JvmVal::Double(x) = val {
            return x;
        }

        panic!("popped value was not double, got {:?}", val);
    }

    pub fn push(self: &mut Frame, val: JvmVal) {
        self.stack.push(val);
    }

    pub fn read_two_byte_index(self: &mut Frame) -> u16 {
        self.ip += 2;
        ((self.code[(self.ip - 1) as usize] as u16) << 8) | (self.code[(self.ip) as usize] as u16)
    }

    pub fn read_four_byte_index(self: &mut Frame) -> u32 {
        self.ip += 4;
        u32::from_be_bytes(
            self.code[self.ip as usize - 3..self.ip as usize + 1]
                .try_into()
                .unwrap(),
        )
    }

    pub fn read_one_byte_index(self: &mut Frame) -> u8 {
        self.ip += 1;
        self.code[self.ip as usize]
    }
}
