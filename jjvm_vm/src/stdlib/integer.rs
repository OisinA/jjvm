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
            "valueOf" => IntegerClass::value_of,
            "intValue" => IntegerClass::int_value,
            _ => panic!("method not found {}", method),
        }
    }
}

impl IntegerClass {
    fn init(vm: &mut VM, vals: Vec<JvmVal>) -> JvmVal {
        let mut v = HashMap::new();

        v.insert("value".to_string(), vals[0].clone());

        let ptr = vm
            .heap
            .alloc(JvmVal::Class("java/lang/Integer".to_string(), v));

        JvmVal::Reference(ptr)
    }

    fn parse_int(_: &mut VM, vals: Vec<JvmVal>) -> JvmVal {
        match &vals[0] {
            JvmVal::String(s) => {
                let s = s.as_str();
                let i = s.parse::<i32>().unwrap();
                JvmVal::Int(i)
            }
            _ => panic!("parseInt expects a string"),
        }
    }

    fn value_of(_: &mut VM, vals: Vec<JvmVal>) -> JvmVal {
        match &vals[0] {
            JvmVal::String(s) => {
                let s = s.as_str();
                let i = s.parse::<i32>().unwrap();
                JvmVal::Int(i)
            }
            _ => panic!("valueOf expects a string, got {:?}", vals[0]),
        }
    }

    fn int_value(vm: &mut VM, vals: Vec<JvmVal>) -> JvmVal {
        let ptr = match vals[0].clone() {
            JvmVal::Reference(ptr) => ptr,
            JvmVal::Int(v) => return JvmVal::Int(v),
            _ => panic!("invalid argument, got {:?}", vals[0]),
        };
        let scanner_values = match vm.heap.fetch(ptr) {
            JvmVal::Class(_, v) => v,
            _ => panic!("invalid argument"),
        };

        let value = match scanner_values.get("value") {
            Some(v) => v,
            None => panic!("invalid argument"),
        };

        return value.clone();
    }
}
