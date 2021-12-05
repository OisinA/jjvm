use std::{
    collections::HashMap,
    hash::{Hash, Hasher},
};

use jjvm_loader::class::Field;

use crate::{jvm_val::JvmVal, vm::VM};

use super::class::BuiltinClass;

use java_utils::HashCode;

pub struct StringClass {}

impl BuiltinClass for StringClass {
    fn get_class_name(self) -> String {
        "java/lang/String".to_string()
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
            "<init>" => StringClass::init,
            "split" => StringClass::split,
            "hashCode" => StringClass::hashcode,
            "equals" => StringClass::equals,
            _ => panic!("method not found {}", method),
        }
    }
}

impl StringClass {
    fn init(vm: &mut VM, vals: Vec<JvmVal>) -> JvmVal {
        println!("{:?}", vals);
        JvmVal::Null
    }

    fn split(vm: &mut VM, vals: Vec<JvmVal>) -> JvmVal {
        let str = match &vals[0] {
            JvmVal::String(s) => s,
            _ => panic!("split expects string"),
        };

        let result = str
            .split(" ")
            .map(|s| s.to_string())
            .map(|s| JvmVal::String(s))
            .collect::<Vec<JvmVal>>();

        let mut ptrs = vec![];
        for str in result {
            let ptr = vm.heap.alloc(str);
            ptrs.push(JvmVal::Reference(ptr));
        }

        let ptr = vm.heap.alloc(JvmVal::Array(ptrs));

        JvmVal::Reference(ptr)
    }

    fn hashcode(vm: &mut VM, vals: Vec<JvmVal>) -> JvmVal {
        let str = match &vals[0] {
            JvmVal::String(s) => s.clone(),
            JvmVal::Reference(v) => match vm.heap.fetch(*v) {
                JvmVal::String(s) => s.clone(),
                _ => panic!("hashcode expects string"),
            },
            _ => panic!("hashcode expects string, got {:?}", vals[0]),
        };

        JvmVal::Int(str.as_str().hash_code())
    }

    fn equals(vm: &mut VM, vals: Vec<JvmVal>) -> JvmVal {
        let str = match &vals[0] {
            JvmVal::String(s) => s.clone(),
            JvmVal::Reference(v) => match vm.heap.fetch(*v) {
                JvmVal::String(s) => s.clone(),
                _ => panic!("equals expects string"),
            },
            _ => panic!("equals expects string, got {:?}", vals[0]),
        };

        let compare = match &vals[1] {
            JvmVal::String(s) => s.clone(),
            _ => panic!("equals expects string, got {:?}", vals[1]),
        };

        JvmVal::Int(if str == compare { 1 } else { 0 })
    }
}
