use std::collections::HashMap;

use jjvm_loader::class::Field;

use crate::{jvm_val::JvmVal, vm::VM};

use super::class::BuiltinClass;

pub struct IntegerClass {}

impl BuiltinClass for IntegerClass {
    fn get_class_name(self) -> String {
        "java/lang/Integer".to_string()
    }

    fn get_fields(&self) -> Vec<Field> {
        let fields = vec![Field {
            flags: 0x0001,
            name: "value".to_string(),
            descriptor: "LI;".to_string(),
            attributes: vec![],
        }];

        return fields;
    }

    fn get_method(&self, method: String) -> fn(&mut VM, Vec<JvmVal>) -> JvmVal {
        match method.as_str() {
            "<init>" => IntegerClass::init,
            "parseInt" => IntegerClass::parse_int,
            _ => panic!("method not found {}", method),
        }
    }
}

impl IntegerClass {
    fn init(vm: &mut VM, vals: Vec<JvmVal>) -> JvmVal {
        println!("{:?}", vals);
        JvmVal::Null
    }

    fn parse_int(vm: &mut VM, vals: Vec<JvmVal>) -> JvmVal {
        match &vals[0] {
            JvmVal::String(s) => {
                let s = s.as_str();
                let i = s.parse::<i32>().unwrap();
                JvmVal::Int(i)
            }
            _ => panic!("parseInt expects a string"),
        }
    }
}
